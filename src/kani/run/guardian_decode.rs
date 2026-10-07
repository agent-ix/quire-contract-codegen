// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX
//! Borrowed JSON primitives for the guardian's strict schema owners (FR-034).
//!
//! This module owns grammar only: no framing, schema inventories, authentication or cause
//! classification. One caller-prepared scratch array is reused for arbitrary nested values.
//! Grammar success and refusal do not allocate. Owners separately retain and charge their
//! payload, decoded records and output storage, and preserve this actual error when applicable.

use std::{
    error::Error,
    fmt,
    mem::{size_of, size_of_val},
};

use super::control::CONTROL_BYTES;

/// The primitive operation or owning schema check that actually refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DecodeSite {
    Input,
    Object,
    Array,
    Field,
    String,
    Unsigned,
    Signed,
    Boolean,
    Null,
    Value,
    End,
    Storage,
}

/// Fixed grammar and schema-check failures; never input-derived diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DecodeCause {
    EmptyInput,
    InputBound,
    UnexpectedEnd,
    UnexpectedToken,
    InvalidSeparator,
    InvalidEscape,
    InvalidUnicode,
    InvalidUtf8,
    InvalidNumber,
    IntegerOverflow,
    TrailingBytes,
    UnknownField,
    DuplicateField,
    MissingField,
    InvalidValue,
    StorageBound,
}

/// An actual fixed decoder/check error with no dynamic payload or reconstructed source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DecodeError {
    site: DecodeSite,
    cause: DecodeCause,
}

impl DecodeError {
    pub(super) const fn new(site: DecodeSite, cause: DecodeCause) -> Self {
        Self { site, cause }
    }

    pub(super) const fn site(self) -> DecodeSite {
        self.site
    }

    pub(super) const fn cause(self) -> DecodeCause {
        self.cause
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(output, "guardian JSON {:?}: {:?}", self.site, self.cause)
    }
}

impl Error for DecodeError {}

#[derive(Clone, Copy)]
#[repr(u8)]
enum ContainerState {
    ArrayFirst,
    ArrayNext,
    ArrayAfter,
    ObjectFirst,
    ObjectNext,
    ObjectAfter,
}

/// One workspace, prepared before role creation and reborrowed by nested decoders.
///
/// Default's inline array initializer may occupy a named temporary before its final placement.
/// The owner must charge actual initialization/placement storage; this layout is not a measured
/// native-stack high-water bound. No scanner or nested decoder constructs another Scratch.
pub(super) struct Scratch {
    states: [ContainerState; CONTROL_BYTES],
}

impl Default for Scratch {
    fn default() -> Self {
        Self {
            states: [ContainerState::ArrayFirst; CONTROL_BYTES],
        }
    }
}

#[derive(Clone, Copy)]
enum SequenceState {
    First,
    After,
    Closed,
}

/// Owner-held scalar state for one object; field uniqueness remains the schema owner's duty.
pub(super) struct ObjectState {
    state: SequenceState,
}

/// Owner-held scalar state for one array.
pub(super) struct ArrayState {
    state: SequenceState,
}

/// Visit an ordinary named record in its owning object or positional sequence grammar.
/// The declaration supplies field order and lookup; custom map-only schemas do not use this.
/// No subtree prevalidation, rewind, allocation or additional scratch is performed.
pub(super) fn record<'input, F: Copy>(
    decoder: &mut Decoder<'input, '_>,
    declared: &[F],
    lookup: impl Fn(Text<'input>) -> Option<F>,
    mut visit: impl FnMut(&mut Decoder<'input, '_>, F) -> Result<(), DecodeError>,
) -> Result<(), DecodeError> {
    let error = |cause| DecodeError::new(DecodeSite::Field, cause);
    match decoder.peek_kind()? {
        ValueKind::Object => {
            let mut object = decoder.begin_object()?;
            while let Some(name) = decoder.next_field(&mut object)? {
                visit(
                    decoder,
                    lookup(name).ok_or_else(|| error(DecodeCause::UnknownField))?,
                )?;
            }
        }
        ValueKind::Array => {
            let mut array = decoder.begin_array()?;
            for field in declared {
                if !decoder.next_element(&mut array)? {
                    return Err(error(DecodeCause::MissingField));
                }
                visit(decoder, *field)?;
            }
            if decoder.next_element(&mut array)? {
                return Err(error(DecodeCause::InvalidValue));
            }
        }
        ValueKind::String | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => {
            return Err(error(DecodeCause::UnexpectedToken));
        }
    }
    Ok(())
}

/// A position in this immutable input; schema visitors may retain their consumed byte range.
/// This mark grants no grammar, sender, frame/state or materialization authority.
#[derive(Clone, Copy)]
pub(super) struct CursorMark<'input> {
    input: &'input [u8],
    position: usize,
}

/// Exactly one validated borrowed value, excluding surrounding whitespace.
#[derive(Clone, Copy)]
pub(super) struct ValueSlice<'input> {
    bytes: &'input [u8],
}

impl<'input> ValueSlice<'input> {
    pub(super) fn bytes(self) -> &'input [u8] {
        self.bytes
    }
}

/// A validated JSON string's encoded interior, without its quotes.
#[derive(Clone, Copy)]
pub(super) struct Text<'input> {
    encoded: &'input [u8],
}

impl<'input> Text<'input> {
    pub(super) fn equals(self, expected: &str) -> bool {
        self.chars().eq(expected.chars())
    }

    pub(super) fn as_unescaped_str(self) -> Option<&'input str> {
        if self.encoded.contains(&b'\\') {
            None
        } else {
            std::str::from_utf8(self.encoded).ok()
        }
    }

    pub(super) fn chars(self) -> TextChars<'input> {
        TextChars {
            encoded: self.encoded,
            position: 0,
        }
    }

    /// Refuse before mutation if either the owner's limit or its real capacity is insufficient.
    pub(super) fn append_to(
        self,
        output: &mut String,
        existing_limit: usize,
    ) -> Result<(), DecodeError> {
        let decoded = self.chars().try_fold(0_usize, |length, character| {
            length.checked_add(character.len_utf8()).ok_or_else(storage)
        })?;
        let final_length = output.len().checked_add(decoded).ok_or_else(storage)?;
        if final_length > existing_limit || final_length > output.capacity() {
            return Err(storage());
        }
        for character in self.chars() {
            output.push(character);
        }
        Ok(())
    }
}

/// Fixed scalar iterator state over an already validated string.
pub(super) struct TextChars<'input> {
    encoded: &'input [u8],
    position: usize,
}

impl Iterator for TextChars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let byte = *self.encoded.get(self.position)?;
        if byte == b'\\' {
            self.position = self.position.checked_add(1)?;
            // Text's private construction has already validated this exact escape.
            return escape(self.encoded, &mut self.position).ok();
        }
        let width = match byte {
            0..=0x7f => 1,
            0xc2..=0xdf => 2,
            0xe0..=0xef => 3,
            0xf0..=0xf4 => 4,
            _ => return None,
        };
        let end = self.position.checked_add(width)?;
        let text = std::str::from_utf8(self.encoded.get(self.position..end)?).ok()?;
        self.position = end;
        text.chars().next()
    }
}

struct ScanState {
    start: usize,
    depth: usize,
}

struct NumberToken<'input> {
    bytes: &'input [u8],
    integral: bool,
}

/// The next scalar/container category only, without validating or consuming its value.
/// It grants no schema, semantic field, framing or authentication authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ValueKind {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}

/// One payload cursor borrowing the owner's sole workspace.
pub(super) struct Decoder<'input, 'scratch> {
    input: &'input [u8],
    position: usize,
    scratch: &'scratch mut Scratch,
}

impl<'input, 'scratch> Decoder<'input, 'scratch> {
    pub(super) fn new(
        input: &'input [u8],
        scratch: &'scratch mut Scratch,
    ) -> Result<Self, DecodeError> {
        if input.is_empty() {
            return Err(DecodeError::new(DecodeSite::Input, DecodeCause::EmptyInput));
        }
        if input.len() > CONTROL_BYTES {
            return Err(DecodeError::new(DecodeSite::Input, DecodeCause::InputBound));
        }
        std::str::from_utf8(input)
            .map_err(|_| DecodeError::new(DecodeSite::Input, DecodeCause::InvalidUtf8))?;
        Ok(Self {
            input,
            position: 0,
            scratch,
        })
    }

    /// Look only at the next non-whitespace byte, preserving all value bytes and the cursor.
    /// Schema alternatives can therefore observe their first actual fault before later syntax.
    pub(super) fn peek_kind(&self) -> Result<ValueKind, DecodeError> {
        let byte = self
            .input
            .get(self.position..)
            .and_then(|remaining| {
                remaining
                    .iter()
                    .find(|byte| !matches!(**byte, b' ' | b'\n' | b'\r' | b'\t'))
            })
            .copied()
            .ok_or_else(|| DecodeError::new(DecodeSite::Value, DecodeCause::UnexpectedEnd))?;
        match byte {
            b'{' => Ok(ValueKind::Object),
            b'[' => Ok(ValueKind::Array),
            b'"' => Ok(ValueKind::String),
            b'-' | b'0'..=b'9' => Ok(ValueKind::Number),
            b't' | b'f' => Ok(ValueKind::Boolean),
            b'n' => Ok(ValueKind::Null),
            _ => Err(DecodeError::new(
                DecodeSite::Value,
                DecodeCause::UnexpectedToken,
            )),
        }
    }

    /// Reborrow this SAME workspace for a sequential operation while preserving the original
    /// cursor. Materialization of an already validated immutable span may use it after frame
    /// authentication; no additional Scratch or competing control consumer is constructed.
    /// Scanner calls own their scratch state only for the call, while Object/ArrayState is
    /// explicitly held by the schema. Native-stack simultaneity remains the owner's charge.
    pub(super) fn with_scratch<T, E>(
        &mut self,
        work: impl FnOnce(&mut Scratch) -> Result<T, E>,
    ) -> Result<T, E> {
        work(self.scratch)
    }

    /// Record this current input position without consuming or prevalidating a subtree.
    pub(super) fn mark(&self) -> CursorMark<'input> {
        CursorMark {
            input: self.input,
            position: self.position,
        }
    }

    /// Borrow already consumed bytes from this SAME input, with checked range and identity.
    /// The schema must have visited them successfully; the range itself proves no grammar.
    pub(super) fn consumed_since(
        &self,
        mark: CursorMark<'input>,
    ) -> Result<&'input [u8], DecodeError> {
        if !std::ptr::eq(mark.input, self.input) {
            return Err(DecodeError::new(
                DecodeSite::Input,
                DecodeCause::InvalidValue,
            ));
        }
        self.input
            .get(mark.position..self.position)
            .ok_or_else(|| DecodeError::new(DecodeSite::Input, DecodeCause::InvalidValue))
    }

    pub(super) fn begin_object(&mut self) -> Result<ObjectState, DecodeError> {
        self.expect(b'{', DecodeSite::Object)?;
        Ok(ObjectState {
            state: SequenceState::First,
        })
    }

    pub(super) fn next_field(
        &mut self,
        state: &mut ObjectState,
    ) -> Result<Option<Text<'input>>, DecodeError> {
        if !self.next_item(&mut state.state, b'}', DecodeSite::Field)? {
            return Ok(None);
        }
        let name = self.string()?;
        self.expect(b':', DecodeSite::Field)?;
        Ok(Some(name))
    }

    pub(super) fn begin_array(&mut self) -> Result<ArrayState, DecodeError> {
        self.expect(b'[', DecodeSite::Array)?;
        Ok(ArrayState {
            state: SequenceState::First,
        })
    }

    pub(super) fn next_element(&mut self, state: &mut ArrayState) -> Result<bool, DecodeError> {
        self.next_item(&mut state.state, b']', DecodeSite::Array)
    }

    fn next_item(
        &mut self,
        state: &mut SequenceState,
        closing: u8,
        site: DecodeSite,
    ) -> Result<bool, DecodeError> {
        let byte = self.required(site)?;
        match state {
            SequenceState::Closed => {
                return Err(DecodeError::new(site, DecodeCause::InvalidSeparator));
            }
            SequenceState::First => {
                if byte == closing {
                    self.advance(site)?;
                    *state = SequenceState::Closed;
                    return Ok(false);
                }
            }
            SequenceState::After => {
                if byte == closing {
                    self.advance(site)?;
                    *state = SequenceState::Closed;
                    return Ok(false);
                }
                if byte != b',' {
                    return Err(DecodeError::new(site, DecodeCause::InvalidSeparator));
                }
                self.advance(site)?;
                if self.required(site)? == closing {
                    return Err(DecodeError::new(site, DecodeCause::InvalidSeparator));
                }
            }
        }
        *state = SequenceState::After;
        Ok(true)
    }

    /// Ordinary externally tagged unit-enum grammar: a name string or one name/null map.
    /// This grants no schema label authority; the owning enum validates the returned name.
    /// Custom string-only seeds must continue using string(), rather than this operation.
    pub(super) fn unit_variant(&mut self) -> Result<Text<'input>, DecodeError> {
        match self.peek_kind()? {
            ValueKind::String => self.string(),
            ValueKind::Object => {
                let mut object = self.begin_object()?;
                let name = self.next_field(&mut object)?.ok_or_else(|| {
                    DecodeError::new(DecodeSite::Field, DecodeCause::MissingField)
                })?;
                self.null()?;
                if self.next_field(&mut object)?.is_some() {
                    return Err(DecodeError::new(
                        DecodeSite::Field,
                        DecodeCause::InvalidValue,
                    ));
                }
                Ok(name)
            }
            ValueKind::Array | ValueKind::Number | ValueKind::Boolean | ValueKind::Null => Err(
                DecodeError::new(DecodeSite::String, DecodeCause::UnexpectedToken),
            ),
        }
    }

    pub(super) fn string(&mut self) -> Result<Text<'input>, DecodeError> {
        self.expect(b'"', DecodeSite::String)?;
        let start = self.position;
        loop {
            let byte = *self
                .input
                .get(self.position)
                .ok_or_else(|| DecodeError::new(DecodeSite::String, DecodeCause::UnexpectedEnd))?;
            match byte {
                b'"' => {
                    let encoded = self.input.get(start..self.position).ok_or_else(|| {
                        DecodeError::new(DecodeSite::String, DecodeCause::InvalidValue)
                    })?;
                    self.advance(DecodeSite::String)?;
                    return Ok(Text { encoded });
                }
                b'\\' => {
                    self.advance(DecodeSite::String)?;
                    escape(self.input, &mut self.position)?;
                }
                0..=0x1f => {
                    return Err(DecodeError::new(
                        DecodeSite::String,
                        DecodeCause::UnexpectedToken,
                    ));
                }
                _ => self.advance(DecodeSite::String)?,
            }
        }
    }

    pub(super) fn boolean(&mut self) -> Result<bool, DecodeError> {
        match self.required(DecodeSite::Boolean)? {
            b't' => {
                self.literal(b"true", DecodeSite::Boolean)?;
                Ok(true)
            }
            b'f' => {
                self.literal(b"false", DecodeSite::Boolean)?;
                Ok(false)
            }
            _ => Err(DecodeError::new(
                DecodeSite::Boolean,
                DecodeCause::UnexpectedToken,
            )),
        }
    }

    pub(super) fn null(&mut self) -> Result<(), DecodeError> {
        self.literal(b"null", DecodeSite::Null)
    }

    pub(super) fn take_null(&mut self) -> Result<bool, DecodeError> {
        let original = self.position;
        if self.required(DecodeSite::Null)? != b'n' {
            // A probe does not turn malformed non-null bytes into an absent value. Validate
            // the actual value using the same workspace, then restore the original cursor.
            self.value()?;
            self.position = original;
            return Ok(false);
        }
        self.null()?;
        Ok(true)
    }

    pub(super) fn unsigned(&mut self) -> Result<u64, DecodeError> {
        let token = self.number(DecodeSite::Unsigned)?;
        if !token.integral || token.bytes.first() == Some(&b'-') {
            return Err(DecodeError::new(
                DecodeSite::Unsigned,
                DecodeCause::InvalidNumber,
            ));
        }
        magnitude(token.bytes, DecodeSite::Unsigned)
    }

    pub(super) fn signed(&mut self) -> Result<i64, DecodeError> {
        let token = self.number(DecodeSite::Signed)?;
        if !token.integral {
            return Err(DecodeError::new(
                DecodeSite::Signed,
                DecodeCause::InvalidNumber,
            ));
        }
        let negative = token.bytes.first() == Some(&b'-');
        let digits = if negative {
            token
                .bytes
                .get(1..)
                .ok_or_else(|| DecodeError::new(DecodeSite::Signed, DecodeCause::InvalidNumber))?
        } else {
            token.bytes
        };
        let value = magnitude(digits, DecodeSite::Signed)?;
        // The owning serde_json integer grammar routes -0 through its float visitor, which
        // signed integer fields refuse. Preserve that schema behavior without a float parser.
        if negative && value == 0 {
            return Err(DecodeError::new(
                DecodeSite::Signed,
                DecodeCause::InvalidNumber,
            ));
        }
        if negative && value == i64::MIN.unsigned_abs() {
            return Ok(i64::MIN);
        }
        let value = i64::try_from(value)
            .map_err(|_| DecodeError::new(DecodeSite::Signed, DecodeCause::IntegerOverflow))?;
        Ok(if negative { -value } else { value })
    }

    pub(super) fn byte(&mut self) -> Result<u8, DecodeError> {
        u8::try_from(self.unsigned()?)
            .map_err(|_| DecodeError::new(DecodeSite::Unsigned, DecodeCause::IntegerOverflow))
    }

    /// Validate an arbitrary nested value with an iterative, byte-bounded syntax stack.
    pub(super) fn value(&mut self) -> Result<ValueSlice<'input>, DecodeError> {
        self.whitespace()?;
        let mut scan = ScanState {
            start: self.position,
            depth: 0,
        };
        self.value_start(&mut scan)?;
        while let Some(index) = scan.depth.checked_sub(1) {
            let state = *self.scratch.states.get(index).ok_or_else(storage)?;
            match state {
                ContainerState::ArrayFirst | ContainerState::ArrayNext => {
                    if matches!(state, ContainerState::ArrayFirst)
                        && self.required(DecodeSite::Value)? == b']'
                    {
                        self.advance(DecodeSite::Value)?;
                        scan.depth = index;
                    } else {
                        self.set_state(index, ContainerState::ArrayAfter)?;
                        self.value_start(&mut scan)?;
                    }
                }
                ContainerState::ObjectFirst | ContainerState::ObjectNext => {
                    if matches!(state, ContainerState::ObjectFirst)
                        && self.required(DecodeSite::Value)? == b'}'
                    {
                        self.advance(DecodeSite::Value)?;
                        scan.depth = index;
                    } else {
                        self.string()?;
                        self.expect(b':', DecodeSite::Value)?;
                        self.set_state(index, ContainerState::ObjectAfter)?;
                        self.value_start(&mut scan)?;
                    }
                }
                ContainerState::ArrayAfter => {
                    self.container_after(&mut scan, index, b']', ContainerState::ArrayNext)?;
                }
                ContainerState::ObjectAfter => {
                    self.container_after(&mut scan, index, b'}', ContainerState::ObjectNext)?;
                }
            }
        }
        self.boundary(DecodeSite::Value, DecodeCause::UnexpectedToken)?;
        let bytes = self
            .input
            .get(scan.start..self.position)
            .ok_or_else(|| DecodeError::new(DecodeSite::Value, DecodeCause::InvalidValue))?;
        Ok(ValueSlice { bytes })
    }

    fn value_start(&mut self, scan: &mut ScanState) -> Result<(), DecodeError> {
        match self.required(DecodeSite::Value)? {
            b'[' => self.push_container(scan, ContainerState::ArrayFirst),
            b'{' => self.push_container(scan, ContainerState::ObjectFirst),
            b'"' => {
                self.string()?;
                self.boundary(DecodeSite::Value, DecodeCause::UnexpectedToken)
            }
            b't' | b'f' => self.boolean().map(|_| ()),
            b'n' => self.null(),
            b'-' | b'0'..=b'9' => self.number(DecodeSite::Value).map(|_| ()),
            _ => Err(DecodeError::new(
                DecodeSite::Value,
                DecodeCause::UnexpectedToken,
            )),
        }
    }

    fn push_container(
        &mut self,
        scan: &mut ScanState,
        state: ContainerState,
    ) -> Result<(), DecodeError> {
        self.advance(DecodeSite::Value)?;
        self.set_state(scan.depth, state)?;
        scan.depth = scan.depth.checked_add(1).ok_or_else(storage)?;
        Ok(())
    }

    fn set_state(&mut self, index: usize, state: ContainerState) -> Result<(), DecodeError> {
        *self.scratch.states.get_mut(index).ok_or_else(storage)? = state;
        Ok(())
    }

    fn container_after(
        &mut self,
        scan: &mut ScanState,
        index: usize,
        closing: u8,
        next: ContainerState,
    ) -> Result<(), DecodeError> {
        match self.required(DecodeSite::Value)? {
            byte if byte == closing => {
                self.advance(DecodeSite::Value)?;
                scan.depth = index;
                Ok(())
            }
            b',' => {
                self.advance(DecodeSite::Value)?;
                self.set_state(index, next)
            }
            _ => Err(DecodeError::new(
                DecodeSite::Value,
                DecodeCause::InvalidSeparator,
            )),
        }
    }

    /// Reborrow the same workspace; the parent cursor is inaccessible for the child lifetime.
    pub(super) fn nested<'borrow>(
        &'borrow mut self,
        value: ValueSlice<'input>,
    ) -> Result<Decoder<'input, 'borrow>, DecodeError> {
        Decoder::new(value.bytes, self.scratch)
    }

    pub(super) fn finish(&mut self) -> Result<(), DecodeError> {
        self.whitespace()?;
        if self.position != self.input.len() {
            return Err(DecodeError::new(
                DecodeSite::End,
                DecodeCause::TrailingBytes,
            ));
        }
        // Object/array state is owner-held. Revalidate with the SAME workspace so a schema
        // reader that reached physical EOF inside an unclosed container cannot report success.
        // This is a second bounded linear scan, not a second cursor or schema authority.
        self.position = 0;
        self.value()?;
        self.whitespace()?;
        if self.position == self.input.len() {
            Ok(())
        } else {
            Err(DecodeError::new(
                DecodeSite::End,
                DecodeCause::TrailingBytes,
            ))
        }
    }

    fn number(&mut self, site: DecodeSite) -> Result<NumberToken<'input>, DecodeError> {
        self.whitespace()?;
        let start = self.position;
        if self.input.get(self.position) == Some(&b'-') {
            self.advance(site)?;
        }
        match self.input.get(self.position) {
            Some(b'0') => self.advance(site)?,
            Some(b'1'..=b'9') => self.digits(site)?,
            None => return Err(DecodeError::new(site, DecodeCause::UnexpectedEnd)),
            _ => return Err(DecodeError::new(site, DecodeCause::InvalidNumber)),
        }
        let mut integral = true;
        if self.input.get(self.position) == Some(&b'.') {
            integral = false;
            self.advance(site)?;
            self.required_digit(site)?;
            self.digits(site)?;
        }
        if matches!(self.input.get(self.position), Some(b'e' | b'E')) {
            integral = false;
            self.advance(site)?;
            if matches!(self.input.get(self.position), Some(b'+' | b'-')) {
                self.advance(site)?;
            }
            self.required_digit(site)?;
            self.digits(site)?;
        }
        self.boundary(site, DecodeCause::InvalidNumber)?;
        let bytes = self
            .input
            .get(start..self.position)
            .ok_or_else(|| DecodeError::new(site, DecodeCause::InvalidNumber))?;
        Ok(NumberToken { bytes, integral })
    }

    fn required_digit(&self, site: DecodeSite) -> Result<(), DecodeError> {
        match self.input.get(self.position) {
            Some(b'0'..=b'9') => Ok(()),
            None => Err(DecodeError::new(site, DecodeCause::UnexpectedEnd)),
            _ => Err(DecodeError::new(site, DecodeCause::InvalidNumber)),
        }
    }

    fn digits(&mut self, site: DecodeSite) -> Result<(), DecodeError> {
        while matches!(self.input.get(self.position), Some(b'0'..=b'9')) {
            self.advance(site)?;
        }
        Ok(())
    }

    fn literal(&mut self, literal: &[u8], site: DecodeSite) -> Result<(), DecodeError> {
        self.whitespace()?;
        for expected in literal {
            match self.input.get(self.position) {
                Some(actual) if actual == expected => self.advance(site)?,
                None => return Err(DecodeError::new(site, DecodeCause::UnexpectedEnd)),
                _ => return Err(DecodeError::new(site, DecodeCause::UnexpectedToken)),
            }
        }
        self.boundary(site, DecodeCause::UnexpectedToken)
    }

    fn boundary(&self, site: DecodeSite, cause: DecodeCause) -> Result<(), DecodeError> {
        match self.input.get(self.position) {
            None | Some(b' ' | b'\t' | b'\r' | b'\n' | b',' | b']' | b'}') => Ok(()),
            _ => Err(DecodeError::new(site, cause)),
        }
    }

    fn expect(&mut self, expected: u8, site: DecodeSite) -> Result<(), DecodeError> {
        if self.required(site)? != expected {
            return Err(DecodeError::new(site, DecodeCause::UnexpectedToken));
        }
        self.advance(site)
    }

    fn required(&mut self, site: DecodeSite) -> Result<u8, DecodeError> {
        self.whitespace()?;
        self.input
            .get(self.position)
            .copied()
            .ok_or_else(|| DecodeError::new(site, DecodeCause::UnexpectedEnd))
    }

    fn whitespace(&mut self) -> Result<(), DecodeError> {
        while matches!(
            self.input.get(self.position),
            Some(b' ' | b'\t' | b'\r' | b'\n')
        ) {
            self.advance(DecodeSite::Input)?;
        }
        Ok(())
    }

    fn advance(&mut self, site: DecodeSite) -> Result<(), DecodeError> {
        self.position = self
            .position
            .checked_add(1)
            .ok_or_else(|| DecodeError::new(site, DecodeCause::InputBound))?;
        Ok(())
    }
}

fn storage() -> DecodeError {
    DecodeError::new(DecodeSite::Storage, DecodeCause::StorageBound)
}

fn magnitude(digits: &[u8], site: DecodeSite) -> Result<u64, DecodeError> {
    digits.iter().try_fold(0_u64, |value, digit| {
        let digit = digit
            .checked_sub(b'0')
            .filter(|digit| *digit <= 9)
            .ok_or_else(|| DecodeError::new(site, DecodeCause::InvalidNumber))?;
        value
            .checked_mul(10)
            .and_then(|value| value.checked_add(u64::from(digit)))
            .ok_or_else(|| DecodeError::new(site, DecodeCause::IntegerOverflow))
    })
}

fn next_byte(input: &[u8], position: &mut usize) -> Result<u8, DecodeError> {
    let byte = input
        .get(*position)
        .copied()
        .ok_or_else(|| DecodeError::new(DecodeSite::String, DecodeCause::UnexpectedEnd))?;
    *position = position.checked_add(1).ok_or_else(storage)?;
    Ok(byte)
}

fn hex4(input: &[u8], position: &mut usize) -> Result<u16, DecodeError> {
    let mut value = 0_u16;
    for _ in 0..4 {
        let digit = match next_byte(input, position)? {
            byte @ b'0'..=b'9' => u16::from(byte - b'0'),
            byte @ b'a'..=b'f' => u16::from(byte - b'a') + 10,
            byte @ b'A'..=b'F' => u16::from(byte - b'A') + 10,
            _ => {
                return Err(DecodeError::new(
                    DecodeSite::String,
                    DecodeCause::InvalidUnicode,
                ))
            }
        };
        value = value
            .checked_mul(16)
            .and_then(|value| value.checked_add(digit))
            .ok_or_else(|| DecodeError::new(DecodeSite::String, DecodeCause::InvalidUnicode))?;
    }
    Ok(value)
}

/// Read after a backslash. Both validation and iteration use this same escape grammar.
fn escape(input: &[u8], position: &mut usize) -> Result<char, DecodeError> {
    let simple = match next_byte(input, position)? {
        b'"' => Some('"'),
        b'\\' => Some('\\'),
        b'/' => Some('/'),
        b'b' => Some('\u{0008}'),
        b'f' => Some('\u{000c}'),
        b'n' => Some('\n'),
        b'r' => Some('\r'),
        b't' => Some('\t'),
        b'u' => None,
        _ => {
            return Err(DecodeError::new(
                DecodeSite::String,
                DecodeCause::InvalidEscape,
            ))
        }
    };
    if let Some(character) = simple {
        return Ok(character);
    }
    let first = hex4(input, position)?;
    let scalar = match first {
        0xd800..=0xdbff => {
            if next_byte(input, position)? != b'\\' || next_byte(input, position)? != b'u' {
                return Err(DecodeError::new(
                    DecodeSite::String,
                    DecodeCause::InvalidUnicode,
                ));
            }
            let second = hex4(input, position)?;
            if !(0xdc00..=0xdfff).contains(&second) {
                return Err(DecodeError::new(
                    DecodeSite::String,
                    DecodeCause::InvalidUnicode,
                ));
            }
            0x10000 + (u32::from(first) - 0xd800) * 0x400 + (u32::from(second) - 0xdc00)
        }
        0xdc00..=0xdfff => {
            return Err(DecodeError::new(
                DecodeSite::String,
                DecodeCause::InvalidUnicode,
            ));
        }
        _ => u32::from(first),
    };
    char::from_u32(scalar)
        .ok_or_else(|| DecodeError::new(DecodeSite::String, DecodeCause::InvalidUnicode))
}

/// Checked inline reservation terms; no ABI numeric guess or duplicated payload/result charge.
///
/// Terms: one Scratch; the outer decoder and one sequential child decoder; one object/array
/// state; one captured span/text/iterator/number record; ScanState; DecodeError; scalar work
/// (one u64, i64, u32, two u16, three usize, bool and char). These are explicit fixed layout
/// terms plus this function's actual fixed reservation table, not compiler frame sizes. Owner-declared deeper schema decoder nesting, initialization
/// temporaries, wrapper/source allocation and decoded result records need separate actual charges.
pub(super) fn decode_bytes() -> Result<u64, DecodeError> {
    let terms = [
        size_of::<Scratch>(),
        size_of::<Decoder<'static, 'static>>(),
        size_of::<Decoder<'static, 'static>>(),
        size_of::<ObjectState>(),
        // unit_variant can retain its own map/name while an enclosing schema stays live.
        size_of::<ObjectState>(),
        size_of::<ArrayState>(),
        size_of::<ValueSlice<'static>>(),
        size_of::<CursorMark<'static>>(),
        size_of::<Result<&'static [u8], DecodeError>>(),
        size_of::<Text<'static>>(),
        size_of::<Text<'static>>(),
        size_of::<TextChars<'static>>(),
        size_of::<NumberToken<'static>>(),
        size_of::<ScanState>(),
        size_of::<DecodeError>(),
        size_of::<ValueKind>(),
        size_of::<u64>(),
        size_of::<i64>(),
        size_of::<u32>(),
        size_of::<u16>(),
        size_of::<u16>(),
        size_of::<usize>(),
        size_of::<usize>(),
        size_of::<usize>(),
        size_of::<bool>(),
        size_of::<char>(),
    ];
    let table_bytes = u64::try_from(size_of_val(&terms)).map_err(|_| storage())?;
    terms.into_iter().try_fold(table_bytes, |sum, term| {
        let term = u64::try_from(term).map_err(|_| storage())?;
        sum.checked_add(term).ok_or_else(storage)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-034-AC-15
    #[test]
    fn consumed_span_is_the_original_visited_range_without_a_subtree_scan() {
        let input = br#"{"bytes":[1,255],"next":0}"#;
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(input, &mut scratch).unwrap();
        let mut object = decoder.begin_object().unwrap();
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("bytes"));
        let mark = decoder.mark();
        let mut array = decoder.begin_array().unwrap();
        assert!(decoder.next_element(&mut array).unwrap());
        assert_eq!(decoder.unsigned().unwrap(), 1);
        assert!(decoder.next_element(&mut array).unwrap());
        assert_eq!(decoder.unsigned().unwrap(), 255);
        assert!(!decoder.next_element(&mut array).unwrap());
        let span = decoder.consumed_since(mark).unwrap();
        assert_eq!(span, b"[1,255]");
        assert!(std::ptr::eq(span.as_ptr(), input[9..16].as_ptr()));
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("next"));
        assert_eq!(decoder.unsigned().unwrap(), 0);
        assert!(decoder.next_field(&mut object).unwrap().is_none());
        decoder.finish().unwrap();
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn consumed_span_refuses_foreign_input_and_a_later_mark_on_a_fresh_cursor() {
        let input = Vec::from(b"[1,2]".as_slice());
        let other = input.clone();
        let mut scratch = Scratch::default();
        let mut foreign_scratch = Scratch::default();
        let foreign = Decoder::new(&other, &mut foreign_scratch).unwrap();
        let mut decoder = Decoder::new(&input, &mut scratch).unwrap();
        assert_eq!(
            decoder.consumed_since(foreign.mark()).unwrap_err().cause(),
            DecodeCause::InvalidValue
        );
        decoder.value().unwrap();
        let later = decoder.mark();
        drop(decoder);
        let fresh = Decoder::new(&input, &mut scratch).unwrap();
        assert_eq!(
            fresh.consumed_since(later).unwrap_err().cause(),
            DecodeCause::InvalidValue
        );
    }

    fn cause(input: &[u8]) -> DecodeCause {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(input, &mut scratch).unwrap();
        match decoder.value() {
            Err(error) => error.cause(),
            Ok(_) => panic!("invalid value admitted"),
        }
    }

    // Grammar units only: no whole schema, authentication, ledger or AC40 claim.
    /// Trace: FR-034-AC-15.
    #[test]
    fn lookahead_preserves_cursor_without_prevalidating_later_structure() {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(br#"  {"bad":"unknown", "later": "#, &mut scratch).unwrap();
        assert_eq!(decoder.peek_kind().unwrap(), ValueKind::Object);
        assert_eq!(decoder.position, 0);
        let mut object = decoder.begin_object().unwrap();
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("bad"));
        assert_eq!(decoder.peek_kind().unwrap(), ValueKind::String);
        assert!(decoder.string().unwrap().equals("unknown"));
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("later"));
        assert_eq!(
            decoder.peek_kind().unwrap_err().cause(),
            DecodeCause::UnexpectedEnd
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn integral_extrema_and_overflow_are_exact() {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(b"18446744073709551615", &mut scratch).unwrap();
        assert_eq!(decoder.unsigned().unwrap(), u64::MAX);
        decoder.finish().unwrap();
        for (input, expected) in [
            (b"-9223372036854775808".as_slice(), i64::MIN),
            (b"9223372036854775807".as_slice(), i64::MAX),
        ] {
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            assert_eq!(decoder.signed().unwrap(), expected);
            decoder.finish().unwrap();
        }
        // Keep this actual edge case rather than treating mathematical zero as its wire type.
        assert!(serde_json::from_slice::<i64>(b"-0").is_err());
        let mut decoder = Decoder::new(b"-0", &mut scratch).unwrap();
        assert_eq!(
            decoder.signed().unwrap_err().cause(),
            DecodeCause::InvalidNumber
        );
        for input in [b"9223372036854775808".as_slice(), b"-9223372036854775809"] {
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            assert_eq!(
                decoder.signed().unwrap_err().cause(),
                DecodeCause::IntegerOverflow
            );
        }
        let mut decoder = Decoder::new(b"18446744073709551616", &mut scratch).unwrap();
        assert_eq!(
            decoder.unsigned().unwrap_err().cause(),
            DecodeCause::IntegerOverflow
        );
        let mut decoder = Decoder::new(b"256", &mut scratch).unwrap();
        assert_eq!(
            decoder.byte().unwrap_err().cause(),
            DecodeCause::IntegerOverflow
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn integral_fields_refuse_json_nonintegers_and_malformed_tokens() {
        let mut scratch = Scratch::default();
        for input in [b"01".as_slice(), b"+1", b"1.0", b"1e2", b"1x", b"-1"] {
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            assert_eq!(
                decoder.unsigned().unwrap_err().cause(),
                DecodeCause::InvalidNumber
            );
        }
        for input in [b"-".as_slice(), b"1.", b"1e", b"1e+"] {
            assert_eq!(cause(input), DecodeCause::UnexpectedEnd);
        }
        let mut decoder = Decoder::new(b"-1.25e+300", &mut scratch).unwrap();
        assert_eq!(decoder.value().unwrap().bytes(), b"-1.25e+300");
        decoder.finish().unwrap();
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn literals_are_strict_and_null_probe_preserves_nonnull_cursor() {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(b" \tfalse", &mut scratch).unwrap();
        assert!(!decoder.take_null().unwrap());
        assert_eq!(decoder.position, 0);
        assert!(!decoder.boolean().unwrap());
        decoder.finish().unwrap();
        let mut decoder = Decoder::new(b"null", &mut scratch).unwrap();
        assert!(decoder.take_null().unwrap());
        decoder.finish().unwrap();
        for input in [b"nullx".as_slice(), b"nulX", b"truex", b"False"] {
            assert_eq!(cause(input), DecodeCause::UnexpectedToken);
        }
        assert_eq!(cause(b"nul"), DecodeCause::UnexpectedEnd);
        for input in [b"truex".as_slice(), b"[1,]", b"+1"] {
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            assert!(
                decoder.take_null().is_err(),
                "malformed non-null probe admitted"
            );
        }
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn text_decodes_utf8_all_escapes_and_surrogate_pairs() {
        let mut scratch = Scratch::default();
        let input = br#""a\"\\\/\b\f\n\r\t\u00e9\ud83d\ude00""#;
        let mut decoder = Decoder::new(input, &mut scratch).unwrap();
        let text = decoder.string().unwrap();
        let expected = "a\"\\/\u{8}\u{c}\n\r\t\u{e9}\u{1f600}";
        assert!(text.equals(expected));
        assert_eq!(text.chars().collect::<String>(), expected);
        assert_eq!(text.as_unescaped_str(), None);
        decoder.finish().unwrap();
        let mut decoder = Decoder::new("\"é😀\"".as_bytes(), &mut scratch).unwrap();
        assert_eq!(decoder.string().unwrap().as_unescaped_str(), Some("é😀"));
        decoder.finish().unwrap();
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn text_refuses_invalid_unicode_escape_control_and_utf8() {
        for input in [
            br#""\ud800x""#.as_slice(),
            br#""\udc00""#,
            br#""\ud800\u0041""#,
            br#""\uZZZZ""#,
        ] {
            assert_eq!(cause(input), DecodeCause::InvalidUnicode);
        }
        assert_eq!(cause(br#""\x""#), DecodeCause::InvalidEscape);
        assert_eq!(cause(b"\"a\n\""), DecodeCause::UnexpectedToken);
        let mut scratch = Scratch::default();
        let error = match Decoder::new(&[b'"', 0xff, b'"'], &mut scratch) {
            Err(error) => error,
            Ok(_) => panic!("invalid UTF8 admitted"),
        };
        assert_eq!(
            error,
            DecodeError::new(DecodeSite::Input, DecodeCause::InvalidUtf8)
        );
    }

    /// Trace: FR-034-AC-32
    #[test]
    fn append_preflights_actual_capacity_and_preserves_output_on_refusal() {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(br#""\u00e9\ud83d\ude00""#, &mut scratch).unwrap();
        let text = decoder.string().unwrap();
        let mut output = String::with_capacity(16);
        output.push('x');
        let capacity = output.capacity();
        assert_eq!(
            text.append_to(&mut output, 6).unwrap_err().cause(),
            DecodeCause::StorageBound
        );
        assert_eq!(output, "x");
        assert_eq!(output.capacity(), capacity);
        text.append_to(&mut output, 7).unwrap();
        assert_eq!(output, "xé😀");
        assert_eq!(output.capacity(), capacity);
        let mut no_capacity = String::new();
        assert_eq!(
            text.append_to(&mut no_capacity, usize::MAX)
                .unwrap_err()
                .cause(),
            DecodeCause::StorageBound
        );
        assert!(no_capacity.is_empty());
        assert_eq!(no_capacity.capacity(), 0);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn spans_and_nested_schema_reborrow_the_single_workspace() {
        let mut scratch = Scratch::default();
        let workspace = std::ptr::from_ref(&scratch);
        let mut decoder =
            Decoder::new(br#" {"a" : [1, {"b": true}], "z":null} "#, &mut scratch).unwrap();
        let mut object = decoder.begin_object().unwrap();
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("a"));
        let span = decoder.value().unwrap();
        assert_eq!(span.bytes(), br#"[1, {"b": true}]"#);
        {
            let mut nested = decoder.nested(span).unwrap();
            assert_eq!(std::ptr::from_ref(nested.scratch), workspace);
            let mut array = nested.begin_array().unwrap();
            assert!(nested.next_element(&mut array).unwrap());
            assert_eq!(nested.byte().unwrap(), 1);
            assert!(nested.next_element(&mut array).unwrap());
            assert_eq!(nested.value().unwrap().bytes(), br#"{"b": true}"#);
            assert!(!nested.next_element(&mut array).unwrap());
            nested.finish().unwrap();
        }
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("z"));
        decoder.null().unwrap();
        assert!(decoder.next_field(&mut object).unwrap().is_none());
        decoder.finish().unwrap();
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn containers_refuse_trailing_separators_mismatched_closes_and_unread_input() {
        for input in [
            b"[1,]".as_slice(),
            b"{\"a\":1,}",
            b"[1 2]",
            b"{\"a\" 1}",
            b"[}",
            b"{]",
        ] {
            let mut scratch = Scratch::default();
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            assert!(decoder.value().is_err(), "malformed container admitted");
        }
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(b"{\"a\":1", &mut scratch).unwrap();
        let mut object = decoder.begin_object().unwrap();
        assert!(decoder
            .next_field(&mut object)
            .unwrap()
            .unwrap()
            .equals("a"));
        assert_eq!(decoder.byte().unwrap(), 1);
        assert_eq!(
            decoder.finish().unwrap_err().cause(),
            DecodeCause::UnexpectedEnd
        );
        let mut decoder = Decoder::new(b"[1,2]", &mut scratch).unwrap();
        let mut array = decoder.begin_array().unwrap();
        assert!(decoder.next_element(&mut array).unwrap());
        assert_eq!(decoder.byte().unwrap(), 1);
        assert_eq!(
            decoder.finish().unwrap_err().cause(),
            DecodeCause::TrailingBytes
        );
        let mut decoder = Decoder::new(b"true false", &mut scratch).unwrap();
        assert!(decoder.boolean().unwrap());
        assert_eq!(
            decoder.finish().unwrap_err().cause(),
            DecodeCause::TrailingBytes
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn all_open_input_uses_original_byte_workspace_and_refuses_without_recursion() {
        assert_eq!(size_of::<ContainerState>(), 1);
        assert_eq!(std::mem::align_of::<Scratch>(), 1);
        assert_eq!(size_of::<Scratch>(), CONTROL_BYTES);
        let input = vec![b'['; CONTROL_BYTES];
        assert_eq!(cause(&input), DecodeCause::UnexpectedEnd);
        let mut complete = vec![b'['; CONTROL_BYTES / 2];
        complete.extend(std::iter::repeat_n(b']', CONTROL_BYTES / 2));
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(&complete, &mut scratch).unwrap();
        assert_eq!(decoder.value().unwrap().bytes(), complete);
        decoder.finish().unwrap();
        let mut scratch = Scratch::default();
        let too_large = vec![b'['; CONTROL_BYTES + 1];
        let error = match Decoder::new(&too_large, &mut scratch) {
            Err(error) => error,
            Ok(_) => panic!("over-bound input admitted"),
        };
        assert_eq!(error.cause(), DecodeCause::InputBound);
        assert!(decode_bytes().unwrap() > u64::try_from(CONTROL_BYTES).unwrap());
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn actual_grammar_error_retains_fixed_identity_without_dynamic_source() {
        let mut scratch = Scratch::default();
        let mut decoder = Decoder::new(b"truex", &mut scratch).unwrap();
        let error = decoder.boolean().unwrap_err();
        assert_eq!(error.site(), DecodeSite::Boolean);
        assert_eq!(error.cause(), DecodeCause::UnexpectedToken);
        assert!(error.source().is_none());
        // Repeated field detection is a schema duty, not invented by generic syntax validation.
        let mut decoder = Decoder::new(br#"{"a":1,"a":2}"#, &mut scratch).unwrap();
        assert_eq!(decoder.value().unwrap().bytes(), br#"{"a":1,"a":2}"#);
        decoder.finish().unwrap();
    }
}
