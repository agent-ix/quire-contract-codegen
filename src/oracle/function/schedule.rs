//! Target-neutral, bounded flat SSA scheduling for FR-021.
//!
//! The input here is a typed projection of admitted V2 expression nodes. It is deliberately
//! separate from rendering and from the former Contract Runtime `exact` adapter. The caller only
//! publishes a schedule after this module has validated the entire reachable body.

use quire_contract_model::CheckedNodeId;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Stable SSA slot assigned in left-to-right postorder.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SsaSlot(
    /// Zero-based instruction ordinal.
    pub u32,
);

/// Stable control-flow block identifier in the target-neutral execution plan.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BlockId(pub u32);

/// Stable occurrence assigned when a body node is entered.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Occurrence(
    /// Stable entry ordinal.
    pub u64,
);

/// Value kind retained from the admitted V2 node's semantic type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ValueKind {
    /// Boolean V2 value.
    Boolean,
    /// Integer V2 value.
    Integer,
}

/// Closed operation set accepted by this first schedule slice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Operation {
    /// Integer addition.
    Add,
    /// Integer subtraction.
    Subtract,
    /// Integer multiplication.
    Multiply,
    /// Equality over Integer operands.
    IntegerEqual,
    /// Inequality over Integer operands.
    IntegerNotEqual,
    /// Equality over Boolean operands.
    BooleanEqual,
    /// Inequality over Boolean operands.
    BooleanNotEqual,
}

impl Operation {
    fn from_identity(identity: &str, node: &CheckedNodeId) -> Result<Self, ScheduleRefusal> {
        match identity {
            "quire.op.integer.add" => Ok(Self::Add),
            "quire.op.integer.sub" => Ok(Self::Subtract),
            "quire.op.integer.mul" => Ok(Self::Multiply),
            "quire.op.integer.eq" => Ok(Self::IntegerEqual),
            "quire.op.integer.ne" => Ok(Self::IntegerNotEqual),
            "quire.op.boolean.eq" => Ok(Self::BooleanEqual),
            "quire.op.boolean.ne" => Ok(Self::BooleanNotEqual),
            _ => Err(ScheduleRefusal::UnsupportedOperation(node.clone())),
        }
    }
}

impl From<CheckedNodeId> for GraphOperand {
    fn from(node: CheckedNodeId) -> Self {
        Self::Node(node)
    }
}

/// Typed input node. References are identities into the admitted graph, never nested terms.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) enum NodeKind {
    Parameter,
    Binary {
        operation_identity: String,
        left: GraphOperand,
        right: GraphOperand,
    },
    Call {
        callee: CheckedNodeId,
        arguments: Vec<GraphOperand>,
    },
}

/// A reference or inline scalar literal in an admitted V2 application.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) enum GraphOperand {
    Node(CheckedNodeId),
    Literal {
        value_kind: ValueKind,
        value: String,
    },
}

/// One admitted graph node projected into the subset this slice can schedule.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct GraphNode {
    pub(crate) id: CheckedNodeId,
    pub(crate) value_kind: ValueKind,
    pub(crate) kind: NodeKind,
}

/// Exact function declaration binding used by nested call nodes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct FunctionBinding {
    pub(crate) declaration: CheckedNodeId,
    pub(crate) parameter_kinds: Vec<ValueKind>,
    pub(crate) result_kind: ValueKind,
}

/// A flat SSA instruction. Operands always name slots emitted earlier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Instruction {
    /// SSA output slot.
    pub output: SsaSlot,
    /// Typed scheduled operation.
    pub operation: ScheduledOperation,
    /// Earlier SSA slots in source operand order.
    pub operands: Vec<Operand>,
    /// V2 source node identity.
    pub node: CheckedNodeId,
    /// Stable operation occurrence ordinal.
    pub occurrence: Occurrence,
}

/// An SSA input or an inline admitted scalar literal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Operand {
    /// Previously emitted SSA value.
    Slot(SsaSlot),
    /// Boolean or Integer literal retained from the V2 source term.
    Literal {
        /// Literal's admitted scalar type.
        value_kind: ValueKind,
        /// Canonical source value text.
        value: String,
    },
}

/// Operations retained in the target-neutral table.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ScheduledOperation {
    /// Function parameter load by declared ordinal.
    Parameter(u32),
    /// Binary operation.
    Binary(Operation),
    /// Call to an exact bound function declaration node.
    Call {
        /// Exact V2 function declaration node bound by this call.
        function: CheckedNodeId,
    },
}

/// Constant-size source location for an operation occurrence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LocationRecord {
    /// Stable operation occurrence ordinal.
    pub occurrence: Occurrence,
    /// V2 source node identity.
    pub node: CheckedNodeId,
    /// Parent operation occurrence, absent at the function body root.
    pub parent: Option<Occurrence>,
    /// Source argument ordinal at the parent operation.
    pub child_ordinal: Option<u32>,
    /// Instruction slot this source location identifies.
    pub instruction: SsaSlot,
}

/// Fully validated schedule and its compact source locations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Schedule {
    /// Flat postorder SSA instructions.
    pub instructions: Vec<Instruction>,
    /// Constant-size parent-linked source locations.
    pub locations: Vec<LocationRecord>,
    /// Ordered charge gates and explicit refusal branches for eventual execution.
    pub control_flow: Vec<ControlBlock>,
    /// First block entered by an executor.
    pub entry: BlockId,
}

/// One block in the target-neutral charge and execution graph.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ControlBlock {
    /// Stable block identifier.
    pub id: BlockId,
    /// Action performed when this block is entered.
    pub action: ControlAction,
    /// Explicit outgoing branch edges.
    pub edges: Vec<ControlEdge>,
}

/// Action represented by one control-flow block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ControlAction {
    /// Charge before executing the named instruction.
    Charge {
        /// Instruction whose operation is being charged.
        instruction: SsaSlot,
    },
    /// Execute the named instruction after its charge was admitted.
    Execute {
        /// Instruction to execute.
        instruction: SsaSlot,
    },
    /// Stop after a denied charge.
    Stop,
}

/// Condition controlling an edge leaving a charge or execution block.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum EdgeCondition {
    /// The preceding charge was admitted.
    ChargeAdmitted,
    /// The preceding charge was denied.
    ChargeDenied,
    /// Unconditional continuation after execution.
    Always,
}

/// Explicit target of a control-flow branch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ControlEdge {
    /// Branch condition.
    pub condition: EdgeCondition,
    /// Destination block.
    pub target: BlockId,
}

/// Typed refusal before any schedule is published.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ScheduleRefusal {
    /// The caller selected `u64::MAX`, whose first denied count cannot be represented.
    InvalidWorkLimit,
    /// Contract IR refused one Scalar/Equality body under a lowering ceiling.
    LoweringWorkExhausted {
        /// Contract IR work ceiling.
        limit: u64,
        /// First denied work count.
        consumed: u64,
    },
    /// Contract IR refused one Scalar/Equality body under its source-byte ceiling.
    LoweringByteLimitExceeded {
        /// Contract IR source-byte ceiling.
        limit: u64,
        /// Canonical bytes consumed by the lowered package.
        consumed: u64,
    },
    /// Contract IR reported a limit kind this generator does not recognize.
    LoweringLimitUnrecognised {
        /// Stable snake_case limit kind.
        limit_kind: &'static str,
        /// Contract IR limit.
        limit: u64,
        /// Contract IR consumed count.
        consumed: u64,
    },
    /// A generation action exceeded the selected work ceiling.
    WorkExhausted {
        /// Caller-selected work ceiling.
        limit: u64,
        /// First denied work count.
        consumed: u64,
    },
    /// A requested V2 node identity is absent from the admitted package.
    MissingNode(CheckedNodeId),
    /// An expression edge reaches a node already active on the traversal stack.
    Cycle(CheckedNodeId),
    /// An expression node is reached from more than one parent.
    SharedExpression(CheckedNodeId),
    /// A parameter node is not part of the supplied function binding.
    UnboundParameter(CheckedNodeId),
    /// A call names no exact bound declaration node.
    UnknownCallee(CheckedNodeId),
    /// A call's argument count does not match the bound declaration.
    WrongArity {
        /// Exact called function declaration.
        function: CheckedNodeId,
        /// Declared parameter count.
        expected: usize,
        /// Supplied argument count.
        found: usize,
    },
    /// A source, operand, parameter or result type does not match its operation.
    WrongType(CheckedNodeId),
    /// The V2 operation identity is outside the supported schedule subset.
    UnsupportedOperation(CheckedNodeId),
    /// The schedule's ordinal or slot counter cannot be represented.
    CounterOverflow,
    /// The graph map key differs from the node's retained identity.
    NodeIdentityMismatch {
        /// Identity used as the graph-map key.
        key: CheckedNodeId,
        /// Identity retained by the graph node.
        stored: CheckedNodeId,
    },
    /// A V2 body does not match the flat function schedule grammar.
    MalformedTerm(CheckedNodeId),
    /// An inline V2 literal is outside the supported Boolean/Integer scalar subset.
    UnsupportedLiteral(CheckedNodeId),
    /// An item names no function binding in the request.
    UnknownFunction(String),
    /// More than one declaration uses the requested function name.
    AmbiguousFunctionName(String),
    /// The same call-node/function request was supplied more than once.
    DuplicateRequest {
        /// Requested call node identity.
        call_node_id: CheckedNodeId,
        /// Requested function name.
        function: String,
    },
    /// The call item's exact callee differs from the requested binding.
    WrongCallBinding {
        /// Requested bound declaration node.
        expected: CheckedNodeId,
        /// Function node referenced by the admitted call item.
        found: CheckedNodeId,
    },
    /// The function declaration does not reference the supplied checked body root.
    WrongBodyBinding {
        /// Exact V2 function declaration.
        function: CheckedNodeId,
        /// Body root supplied by the caller.
        expected: CheckedNodeId,
    },
    /// The complete generated schedule crate exceeds the default source ceiling.
    SourceTooLarge {
        /// Generated byte count.
        bytes: usize,
        /// Default generated-source byte ceiling.
        limit: usize,
    },
    /// Typed schedule tables could not be serialized into generated source.
    SerializationFailure,
}

#[derive(Clone, Copy)]
enum VisitState {
    Active,
    Complete,
}

#[derive(Clone)]
struct Parent {
    occurrence: Occurrence,
    child_ordinal: u32,
}

enum Event {
    Enter {
        node: CheckedNodeId,
        parent: Option<Parent>,
    },
    Continue {
        node: CheckedNodeId,
        occurrence: Occurrence,
        edges: Vec<GraphOperand>,
        next: usize,
        child_ordinal_base: u32,
    },
    Finish {
        node: CheckedNodeId,
        occurrence: Occurrence,
        parent: Option<Parent>,
    },
}

pub(crate) struct WorkCounter {
    limit: u64,
    consumed: u64,
}

impl WorkCounter {
    pub(crate) fn new(limit: u64) -> Result<Self, ScheduleRefusal> {
        if limit == u64::MAX {
            return Err(ScheduleRefusal::InvalidWorkLimit);
        }
        Ok(Self { limit, consumed: 0 })
    }

    /// Charges before the caller performs the action.
    pub(crate) fn charge(&mut self) -> Result<(), ScheduleRefusal> {
        let consumed = self
            .consumed
            .checked_add(1)
            .ok_or(ScheduleRefusal::CounterOverflow)?;
        if consumed > self.limit {
            return Err(ScheduleRefusal::WorkExhausted {
                limit: self.limit,
                consumed,
            });
        }
        self.consumed = consumed;
        Ok(())
    }
}

/// Builds one complete schedule using an explicit heap stack.
#[cfg(test)]
pub(crate) fn build_schedule(
    root: &CheckedNodeId,
    nodes: &BTreeMap<CheckedNodeId, GraphNode>,
    parameters: &BTreeMap<CheckedNodeId, u32>,
    current_function: &FunctionBinding,
    functions: &BTreeMap<CheckedNodeId, FunctionBinding>,
    work_limit: u64,
) -> Result<Schedule, ScheduleRefusal> {
    let mut work = WorkCounter::new(work_limit)?;
    build_schedule_with_work(
        root,
        nodes,
        parameters,
        current_function,
        functions,
        &mut work,
    )
}

/// Builds one schedule using the same budget already charged by graph projection.
pub(crate) fn build_schedule_with_work(
    root: &CheckedNodeId,
    nodes: &BTreeMap<CheckedNodeId, GraphNode>,
    parameters: &BTreeMap<CheckedNodeId, u32>,
    current_function: &FunctionBinding,
    functions: &BTreeMap<CheckedNodeId, FunctionBinding>,
    work: &mut WorkCounter,
) -> Result<Schedule, ScheduleRefusal> {
    let mut bound_slots = BTreeSet::new();
    for index in parameters.values() {
        if !bound_slots.insert(*index)
            || usize::try_from(*index).map_or(true, |index| {
                index >= current_function.parameter_kinds.len()
            })
        {
            return Err(ScheduleRefusal::WrongType(root.clone()));
        }
    }
    let mut stack = vec![Event::Enter {
        node: root.clone(),
        parent: None,
    }];
    let mut states = BTreeMap::<CheckedNodeId, VisitState>::new();
    let mut slots = BTreeMap::<CheckedNodeId, SsaSlot>::new();
    let mut operations = BTreeMap::<CheckedNodeId, Operation>::new();
    let mut instructions = Vec::new();
    let mut locations = Vec::new();
    let mut next_occurrence = 0u64;

    while let Some(event) = stack.pop() {
        match event {
            Event::Enter { node, parent } => {
                work.charge()?;
                let graph_node = nodes
                    .get(&node)
                    .ok_or_else(|| ScheduleRefusal::MissingNode(node.clone()))?;
                if graph_node.id != node {
                    return Err(ScheduleRefusal::NodeIdentityMismatch {
                        key: node,
                        stored: graph_node.id.clone(),
                    });
                }
                if parent.is_none() && graph_node.value_kind != current_function.result_kind {
                    return Err(ScheduleRefusal::WrongType(node.clone()));
                }
                match states.get(&node) {
                    Some(VisitState::Active) => return Err(ScheduleRefusal::Cycle(node)),
                    Some(VisitState::Complete)
                        if matches!(
                            graph_node.kind,
                            NodeKind::Binary { .. } | NodeKind::Call { .. }
                        ) =>
                    {
                        return Err(ScheduleRefusal::SharedExpression(node));
                    }
                    Some(VisitState::Complete) | None => {}
                }
                let occurrence = Occurrence(next_occurrence);
                next_occurrence = next_occurrence
                    .checked_add(1)
                    .ok_or(ScheduleRefusal::CounterOverflow)?;
                states.insert(node.clone(), VisitState::Active);
                if let NodeKind::Binary {
                    operation_identity, ..
                } = &graph_node.kind
                {
                    operations.insert(
                        node.clone(),
                        Operation::from_identity(operation_identity, &node)?,
                    );
                }
                let (edges, child_ordinal_base) = match &graph_node.kind {
                    NodeKind::Parameter => (Vec::new(), 0),
                    NodeKind::Binary { left, right, .. } => (vec![left.clone(), right.clone()], 0),
                    NodeKind::Call { callee, arguments } => {
                        // Argument zero is charged before the binding reference is inspected.
                        work.charge()?;
                        let binding = functions
                            .get(callee)
                            .ok_or_else(|| ScheduleRefusal::UnknownCallee(callee.clone()))?;
                        if binding.declaration != *callee {
                            return Err(ScheduleRefusal::UnknownCallee(callee.clone()));
                        }
                        if binding.parameter_kinds.len() != arguments.len() {
                            return Err(ScheduleRefusal::WrongArity {
                                function: binding.declaration.clone(),
                                expected: binding.parameter_kinds.len(),
                                found: arguments.len(),
                            });
                        }
                        (arguments.clone(), 1)
                    }
                };
                stack.push(Event::Finish {
                    node: node.clone(),
                    occurrence,
                    parent,
                });
                stack.push(Event::Continue {
                    node: node.clone(),
                    occurrence,
                    edges,
                    next: 0,
                    child_ordinal_base,
                });
            }
            Event::Continue {
                node,
                occurrence,
                edges,
                next,
                child_ordinal_base,
            } => {
                if next < edges.len() {
                    work.charge()?;
                    let child = edges[next].clone();
                    stack.push(Event::Continue {
                        node: node.clone(),
                        occurrence,
                        edges,
                        next: next + 1,
                        child_ordinal_base,
                    });
                    if let GraphOperand::Node(child) = child {
                        stack.push(Event::Enter {
                            node: child,
                            parent: Some(Parent {
                                occurrence,
                                child_ordinal: child_ordinal_base
                                    .checked_add(
                                        u32::try_from(next)
                                            .map_err(|_| ScheduleRefusal::CounterOverflow)?,
                                    )
                                    .ok_or(ScheduleRefusal::CounterOverflow)?,
                            }),
                        });
                    }
                }
            }
            Event::Finish {
                node,
                occurrence,
                parent,
            } => {
                let graph_node = nodes
                    .get(&node)
                    .ok_or_else(|| ScheduleRefusal::MissingNode(node.clone()))?;
                let (operation, operands) = match &graph_node.kind {
                    NodeKind::Parameter => {
                        let index = parameters
                            .get(&node)
                            .copied()
                            .ok_or_else(|| ScheduleRefusal::UnboundParameter(node.clone()))?;
                        let expected_kind = usize::try_from(index)
                            .ok()
                            .and_then(|index| current_function.parameter_kinds.get(index))
                            .ok_or_else(|| ScheduleRefusal::WrongType(node.clone()))?;
                        if *expected_kind != graph_node.value_kind {
                            return Err(ScheduleRefusal::WrongType(node.clone()));
                        }
                        (ScheduledOperation::Parameter(index), Vec::new())
                    }
                    NodeKind::Binary { left, right, .. } => {
                        let left_operand = schedule_operand(left, &slots)?;
                        let right_operand = schedule_operand(right, &slots)?;
                        let operation = operations
                            .get(&node)
                            .copied()
                            .ok_or_else(|| ScheduleRefusal::UnsupportedOperation(node.clone()))?;
                        validate_binary(
                            operation,
                            graph_node,
                            operand_kind(left, nodes)?,
                            operand_kind(right, nodes)?,
                        )?;
                        (
                            ScheduledOperation::Binary(operation),
                            vec![left_operand, right_operand],
                        )
                    }
                    NodeKind::Call { callee, arguments } => {
                        let binding = functions
                            .get(callee)
                            .ok_or_else(|| ScheduleRefusal::UnknownCallee(callee.clone()))?;
                        let mut slots_for_args = Vec::with_capacity(arguments.len());
                        for (index, argument) in arguments.iter().enumerate() {
                            let kind = operand_kind(argument, nodes)?;
                            if kind != binding.parameter_kinds[index] {
                                let id = match argument {
                                    GraphOperand::Node(id) => id.clone(),
                                    GraphOperand::Literal { .. } => node.clone(),
                                };
                                return Err(ScheduleRefusal::WrongType(id));
                            }
                            slots_for_args.push(schedule_operand(argument, &slots)?);
                        }
                        if graph_node.value_kind != binding.result_kind {
                            return Err(ScheduleRefusal::WrongType(node.clone()));
                        }
                        (
                            ScheduledOperation::Call {
                                function: binding.declaration.clone(),
                            },
                            slots_for_args,
                        )
                    }
                };
                work.charge()?;
                let output = SsaSlot(
                    u32::try_from(instructions.len())
                        .map_err(|_| ScheduleRefusal::CounterOverflow)?,
                );
                instructions.push(Instruction {
                    output,
                    operation,
                    operands,
                    node: node.clone(),
                    occurrence,
                });
                work.charge()?;
                locations.push(LocationRecord {
                    occurrence,
                    node: node.clone(),
                    parent: parent.as_ref().map(|value| value.occurrence),
                    child_ordinal: parent.map(|value| value.child_ordinal),
                    instruction: output,
                });
                slots.insert(node.clone(), output);
                states.insert(node, VisitState::Complete);
            }
        }
    }

    let (entry, control_flow) = build_control_flow(&instructions)?;
    Ok(Schedule {
        instructions,
        locations,
        control_flow,
        entry,
    })
}

fn build_control_flow(
    instructions: &[Instruction],
) -> Result<(BlockId, Vec<ControlBlock>), ScheduleRefusal> {
    let stop = BlockId(
        u32::try_from(
            instructions
                .len()
                .checked_mul(2)
                .ok_or(ScheduleRefusal::CounterOverflow)?,
        )
        .map_err(|_| ScheduleRefusal::CounterOverflow)?,
    );
    let mut blocks = Vec::with_capacity(instructions.len().saturating_mul(2).saturating_add(1));
    for (index, instruction) in instructions.iter().enumerate() {
        let charge = BlockId(
            u32::try_from(
                index
                    .checked_mul(2)
                    .ok_or(ScheduleRefusal::CounterOverflow)?,
            )
            .map_err(|_| ScheduleRefusal::CounterOverflow)?,
        );
        let execute = BlockId(
            charge
                .0
                .checked_add(1)
                .ok_or(ScheduleRefusal::CounterOverflow)?,
        );
        let next = if index + 1 < instructions.len() {
            Some(BlockId(
                execute
                    .0
                    .checked_add(1)
                    .ok_or(ScheduleRefusal::CounterOverflow)?,
            ))
        } else {
            None
        };
        blocks.push(ControlBlock {
            id: charge,
            action: ControlAction::Charge {
                instruction: instruction.output,
            },
            edges: vec![
                ControlEdge {
                    condition: EdgeCondition::ChargeAdmitted,
                    target: execute,
                },
                ControlEdge {
                    condition: EdgeCondition::ChargeDenied,
                    target: stop,
                },
            ],
        });
        blocks.push(ControlBlock {
            id: execute,
            action: ControlAction::Execute {
                instruction: instruction.output,
            },
            edges: next
                .map(|target| ControlEdge {
                    condition: EdgeCondition::Always,
                    target,
                })
                .into_iter()
                .collect(),
        });
    }
    blocks.push(ControlBlock {
        id: stop,
        action: ControlAction::Stop,
        edges: Vec::new(),
    });
    let entry = if instructions.is_empty() {
        stop
    } else {
        BlockId(0)
    };
    Ok((entry, blocks))
}

fn slot_for(
    node: &CheckedNodeId,
    slots: &BTreeMap<CheckedNodeId, SsaSlot>,
) -> Result<SsaSlot, ScheduleRefusal> {
    slots
        .get(node)
        .copied()
        .ok_or_else(|| ScheduleRefusal::MissingNode(node.clone()))
}

fn schedule_operand(
    operand: &GraphOperand,
    slots: &BTreeMap<CheckedNodeId, SsaSlot>,
) -> Result<Operand, ScheduleRefusal> {
    match operand {
        GraphOperand::Node(node) => slot_for(node, slots).map(Operand::Slot),
        GraphOperand::Literal { value_kind, value } => Ok(Operand::Literal {
            value_kind: *value_kind,
            value: value.clone(),
        }),
    }
}

fn operand_kind(
    operand: &GraphOperand,
    nodes: &BTreeMap<CheckedNodeId, GraphNode>,
) -> Result<ValueKind, ScheduleRefusal> {
    match operand {
        GraphOperand::Node(node) => nodes
            .get(node)
            .map(|graph_node| graph_node.value_kind)
            .ok_or_else(|| ScheduleRefusal::MissingNode(node.clone())),
        GraphOperand::Literal { value_kind, .. } => Ok(*value_kind),
    }
}

fn validate_binary(
    operation: Operation,
    output: &GraphNode,
    left: ValueKind,
    right: ValueKind,
) -> Result<(), ScheduleRefusal> {
    match operation {
        Operation::Add | Operation::Subtract | Operation::Multiply => {
            if output.value_kind != ValueKind::Integer
                || left != ValueKind::Integer
                || right != ValueKind::Integer
            {
                return Err(ScheduleRefusal::WrongType(output.id.clone()));
            }
        }
        Operation::IntegerEqual | Operation::IntegerNotEqual => {
            if output.value_kind != ValueKind::Boolean
                || left != ValueKind::Integer
                || right != ValueKind::Integer
            {
                return Err(ScheduleRefusal::WrongType(output.id.clone()));
            }
        }
        Operation::BooleanEqual | Operation::BooleanNotEqual => {
            if output.value_kind != ValueKind::Boolean
                || left != ValueKind::Boolean
                || right != ValueKind::Boolean
            {
                return Err(ScheduleRefusal::WrongType(output.id.clone()));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u8) -> CheckedNodeId {
        CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: format!("{value:064x}").into(),
        }
    }

    fn node(value: u8, value_kind: ValueKind, kind: NodeKind) -> GraphNode {
        GraphNode {
            id: id(value),
            value_kind,
            kind,
        }
    }

    fn binding(
        value: u8,
        parameter_kinds: Vec<ValueKind>,
        result_kind: ValueKind,
    ) -> FunctionBinding {
        FunctionBinding {
            declaration: id(value),
            parameter_kinds,
            result_kind,
        }
    }

    #[test]
    /// Trace: FR-021-AC-25, FR-021-AC-26, FR-021-AC-27, TC-031.
    fn ac25_nested_add_schedule_is_postorder_and_budget_refusal_publishes_nothing() {
        let root = id(3);
        let left = id(2);
        let mut nodes = BTreeMap::new();
        nodes.insert(id(1), node(1, ValueKind::Integer, NodeKind::Parameter));
        nodes.insert(
            id(2),
            node(
                2,
                ValueKind::Integer,
                NodeKind::Binary {
                    operation_identity: "quire.op.integer.add".to_owned(),
                    left: id(1).into(),
                    right: id(5).into(),
                },
            ),
        );
        nodes.insert(
            id(3),
            node(
                3,
                ValueKind::Integer,
                NodeKind::Binary {
                    operation_identity: "quire.op.integer.mul".to_owned(),
                    left: left.clone().into(),
                    right: id(1).into(),
                },
            ),
        );
        nodes.insert(id(5), node(5, ValueKind::Integer, NodeKind::Parameter));
        let parameters = BTreeMap::from([(id(1), 0), (id(5), 1)]);
        let current = binding(
            90,
            vec![ValueKind::Integer, ValueKind::Integer],
            ValueKind::Integer,
        );
        let exact = build_schedule(&root, &nodes, &parameters, &current, &BTreeMap::new(), 19)
            .expect("the exact charge budget admits the complete schedule");
        assert_eq!(exact.instructions.len(), 5);
        assert_eq!(
            exact.instructions[2].operation,
            ScheduledOperation::Binary(Operation::Add)
        );
        assert_eq!(
            exact.instructions[3].operation,
            ScheduledOperation::Parameter(0),
            "the right operand is reached only after the left operation completes"
        );
        assert_eq!(
            exact.instructions[4].operation,
            ScheduledOperation::Binary(Operation::Multiply),
            "the parent operation follows both operands"
        );
        assert!(exact.instructions.iter().all(|instruction| {
            instruction.operands.iter().all(|operand| match operand {
                Operand::Slot(slot) => slot.0 < instruction.output.0,
                Operand::Literal { .. } => true,
            })
        }));
        assert_eq!(exact.locations.len(), 5);
        let stop = exact
            .control_flow
            .iter()
            .find(|block| block.action == ControlAction::Stop)
            .expect("denied charges terminate in an explicit stop block")
            .id;
        for (index, instruction) in exact.instructions.iter().enumerate() {
            let charge = &exact.control_flow[index * 2];
            let execute = &exact.control_flow[index * 2 + 1];
            assert_eq!(
                charge.action,
                ControlAction::Charge {
                    instruction: instruction.output
                }
            );
            assert_eq!(charge.edges[1].condition, EdgeCondition::ChargeDenied);
            assert_eq!(charge.edges[1].target, stop);
            assert_eq!(charge.edges[0].condition, EdgeCondition::ChargeAdmitted);
            assert_eq!(charge.edges[0].target, execute.id);
            assert_eq!(
                execute.action,
                ControlAction::Execute {
                    instruction: instruction.output
                }
            );
            if let Some(next) = exact.instructions.get(index + 1) {
                assert_eq!(
                    execute.edges[0].target,
                    exact.control_flow[(index + 1) * 2].id
                );
                assert_eq!(
                    exact.control_flow[(index + 1) * 2].action,
                    ControlAction::Charge {
                        instruction: next.output
                    }
                );
            }
        }
        assert_eq!(
            exact.control_flow[5].edges[0].target,
            exact.control_flow[6].id
        );
        assert_eq!(
            exact.control_flow[5].action,
            ControlAction::Execute {
                instruction: exact.instructions[2].output
            }
        );
        assert_eq!(
            exact.control_flow[6].action,
            ControlAction::Charge {
                instruction: exact.instructions[3].output
            },
            "right operand charge follows left completion"
        );
        assert_eq!(
            exact.control_flow[8].action,
            ControlAction::Charge {
                instruction: exact.instructions[4].output
            },
            "parent charge follows right completion"
        );
        assert!(matches!(
            build_schedule(&root, &nodes, &parameters, &current, &BTreeMap::new(), 18),
            Err(ScheduleRefusal::WorkExhausted {
                limit: 18,
                consumed: 19
            })
        ));
    }

    #[test]
    /// Trace: FR-021-AC-25, FR-021-AC-27, TC-031.
    fn ac27_integer_literal_is_an_ordered_inline_schedule_operand() {
        let root = id(3);
        let parameter = id(1);
        let graph = BTreeMap::from([
            (
                parameter.clone(),
                node(1, ValueKind::Integer, NodeKind::Parameter),
            ),
            (
                root.clone(),
                node(
                    3,
                    ValueKind::Integer,
                    NodeKind::Binary {
                        operation_identity: "quire.op.integer.add".to_owned(),
                        left: GraphOperand::Literal {
                            value_kind: ValueKind::Integer,
                            value: "5".to_owned(),
                        },
                        right: parameter.clone().into(),
                    },
                ),
            ),
        ]);
        let schedule = build_schedule(
            &root,
            &graph,
            &BTreeMap::from([(parameter, 0)]),
            &binding(90, vec![ValueKind::Integer], ValueKind::Integer),
            &BTreeMap::new(),
            8,
        )
        .expect("literal and reference operands are both retained");
        assert_eq!(
            schedule.instructions.last().unwrap().operands,
            vec![
                Operand::Literal {
                    value_kind: ValueKind::Integer,
                    value: "5".to_owned(),
                },
                Operand::Slot(SsaSlot(0)),
            ]
        );
    }

    #[test]
    /// Trace: FR-021-AC-27, TC-031.
    fn equality_operation_identity_preserves_its_scalar_domain() {
        assert_eq!(
            Operation::from_identity("quire.op.integer.eq", &id(3)).unwrap(),
            Operation::IntegerEqual
        );
        assert_eq!(
            Operation::from_identity("quire.op.boolean.eq", &id(3)).unwrap(),
            Operation::BooleanEqual
        );
        let root = id(3);
        let graph = BTreeMap::from([
            (id(1), node(1, ValueKind::Boolean, NodeKind::Parameter)),
            (id(2), node(2, ValueKind::Boolean, NodeKind::Parameter)),
            (
                root.clone(),
                node(
                    3,
                    ValueKind::Boolean,
                    NodeKind::Binary {
                        operation_identity: "quire.op.integer.eq".to_owned(),
                        left: id(1).into(),
                        right: id(2).into(),
                    },
                ),
            ),
        ]);
        assert!(matches!(
            build_schedule(
                &root,
                &graph,
                &BTreeMap::from([(id(1), 0), (id(2), 1)]),
                &binding(
                    90,
                    vec![ValueKind::Boolean, ValueKind::Boolean],
                    ValueKind::Boolean
                ),
                &BTreeMap::new(),
                20
            ),
            Err(ScheduleRefusal::WrongType(node_id)) if node_id == root
        ));
    }

    #[test]
    /// Trace: FR-021-AC-27, TC-031.
    fn ac27_call_uses_exact_declaration_identity_and_rejects_mismatched_binding() {
        let root = id(4);
        let arg = id(1);
        let callee = id(8);
        let graph = BTreeMap::from([
            (
                arg.clone(),
                node(1, ValueKind::Integer, NodeKind::Parameter),
            ),
            (
                root.clone(),
                node(
                    4,
                    ValueKind::Integer,
                    NodeKind::Call {
                        callee: callee.clone(),
                        arguments: vec![arg.clone().into()],
                    },
                ),
            ),
        ]);
        let params = BTreeMap::from([(arg, 0)]);
        let functions = BTreeMap::from([(
            callee.clone(),
            binding(8, vec![ValueKind::Integer], ValueKind::Integer),
        )]);
        let current = binding(4, vec![ValueKind::Integer], ValueKind::Integer);
        let schedule =
            build_schedule(&root, &graph, &params, &current, &functions, 10).expect("bound call");
        assert_eq!(
            schedule.instructions[1].operation,
            ScheduledOperation::Call { function: callee }
        );
        assert_eq!(schedule.locations[0].child_ordinal, Some(1));

        let wrong = BTreeMap::from([(
            id(9),
            binding(9, vec![ValueKind::Integer], ValueKind::Integer),
        )]);
        assert!(matches!(
            build_schedule(&root, &graph, &params, &current, &wrong, 10),
            Err(ScheduleRefusal::UnknownCallee(_))
        ));
    }

    #[test]
    /// Trace: FR-021-AC-27, TC-031.
    fn ac27_unknown_operation_and_mismatched_operand_types_refuse_before_schedule() {
        let root = id(3);
        let unsupported = BTreeMap::from([(
            root.clone(),
            node(
                3,
                ValueKind::Integer,
                NodeKind::Binary {
                    operation_identity: "quire.op.integer.div".to_owned(),
                    left: id(1).into(),
                    right: id(2).into(),
                },
            ),
        )]);
        let current = binding(90, vec![], ValueKind::Integer);
        assert!(matches!(
            build_schedule(
                &root,
                &unsupported,
                &BTreeMap::new(),
                &current,
                &BTreeMap::new(),
                20
            ),
            Err(ScheduleRefusal::UnsupportedOperation(node_id)) if node_id == root
        ));

        let mistyped = BTreeMap::from([
            (id(1), node(1, ValueKind::Boolean, NodeKind::Parameter)),
            (id(2), node(2, ValueKind::Integer, NodeKind::Parameter)),
            (
                root.clone(),
                node(
                    3,
                    ValueKind::Integer,
                    NodeKind::Binary {
                        operation_identity: "quire.op.integer.add".to_owned(),
                        left: id(1).into(),
                        right: id(2).into(),
                    },
                ),
            ),
        ]);
        assert!(matches!(
            build_schedule(
                &root,
                &mistyped,
                &BTreeMap::from([(id(1), 0), (id(2), 1)]),
                &binding(90, vec![ValueKind::Boolean, ValueKind::Integer], ValueKind::Integer),
                &BTreeMap::new(),
                20
            ),
            Err(ScheduleRefusal::WrongType(node_id)) if node_id == root
        ));
    }

    #[test]
    /// Trace: FR-021-AC-25, TC-031.
    fn ac25_max_limit_is_rejected_before_any_graph_action() {
        let root = id(1);
        let nodes = BTreeMap::from([(
            root.clone(),
            node(1, ValueKind::Integer, NodeKind::Parameter),
        )]);
        assert_eq!(
            build_schedule(
                &root,
                &nodes,
                &BTreeMap::new(),
                &binding(9, vec![], ValueKind::Integer),
                &BTreeMap::new(),
                u64::MAX
            ),
            Err(ScheduleRefusal::InvalidWorkLimit)
        );
    }

    #[test]
    /// Trace: FR-021-AC-27, TC-031.
    fn ac27_cycle_shared_node_and_unbound_parameter_refuse_without_a_schedule() {
        let root = id(2);
        let cycle = BTreeMap::from([(
            root.clone(),
            node(
                2,
                ValueKind::Integer,
                NodeKind::Binary {
                    operation_identity: "quire.op.integer.add".to_owned(),
                    left: root.clone().into(),
                    right: root.clone().into(),
                },
            ),
        )]);
        let current = binding(90, vec![], ValueKind::Integer);
        assert!(matches!(
            build_schedule(
                &root,
                &cycle,
                &BTreeMap::new(),
                &current,
                &BTreeMap::new(),
                20
            ),
            Err(ScheduleRefusal::Cycle(_))
        ));

        let unbound = BTreeMap::from([(
            root.clone(),
            node(2, ValueKind::Integer, NodeKind::Parameter),
        )]);
        assert!(matches!(
            build_schedule(
                &root,
                &unbound,
                &BTreeMap::new(),
                &current,
                &BTreeMap::new(),
                20
            ),
            Err(ScheduleRefusal::UnboundParameter(_))
        ));

        let shared = BTreeMap::from([
            (
                id(1),
                node(
                    1,
                    ValueKind::Integer,
                    NodeKind::Binary {
                        operation_identity: "quire.op.integer.add".to_owned(),
                        left: id(3).into(),
                        right: id(4).into(),
                    },
                ),
            ),
            (
                id(2),
                node(
                    2,
                    ValueKind::Integer,
                    NodeKind::Binary {
                        operation_identity: "quire.op.integer.add".to_owned(),
                        left: id(1).into(),
                        right: id(1).into(),
                    },
                ),
            ),
            (id(3), node(3, ValueKind::Integer, NodeKind::Parameter)),
            (id(4), node(4, ValueKind::Integer, NodeKind::Parameter)),
        ]);
        assert!(matches!(
            build_schedule(
                &id(2),
                &shared,
                &BTreeMap::from([(id(3), 0), (id(4), 1)]),
                &binding(
                    90,
                    vec![ValueKind::Integer, ValueKind::Integer],
                    ValueKind::Integer
                ),
                &BTreeMap::new(),
                20
            ),
            Err(ScheduleRefusal::SharedExpression(_))
        ));
    }
}
