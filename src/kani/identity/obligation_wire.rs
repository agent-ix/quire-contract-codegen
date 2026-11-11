//! The persisted wire form of a state-frame harness's QSL [`ObligationIdentity`]
//! (FR-015-AC-138).
//!
//! The pinned `qsl_replay::ObligationIdentity` derives no serde, so this form is CG's own:
//! `{"node": {"package", "path", "ordinal"}, "model": {"identity"}, "ordinal"}`, `model` absent
//! when the identity has none. A path segment is `{"text": <string>}` or
//! `{"node": <node object>}`; a `Node` segment nests a whole key and is never flattened to text.
//! `node` and `ordinal` are required: a record without them is refused, never defaulted
//! (FR-015-AC-139).

use qsl_replay::{ModelIdentity, ObligationIdentity};
use quire_exact::{NodeKey, PathSegment};
use serde::{
    de::Error as _, ser::Error as _, ser::SerializeMap, ser::SerializeSeq, Deserialize,
    Deserializer, Serialize, Serializer,
};

/// The deepest node nesting the wire form carries (the obligation's own node is level 1).
///
/// `serde_json` refuses a document nested past 128 containers on read. Each node level adds
/// three (the node object, its `path` array and the segment object) below the three that wrap
/// the first node (`RecordView`, `identity`, `obligation`), so the read limit is first reached
/// near level 42. A bound of 32 leaves about nine levels of margin: a deeper key would be
/// written but never read back, so the writer refuses it as
/// `StateFrameRefusal::RecordSerialization` and the reader refuses it too (FR-015-AC-138).
const MAX_NODE_DEPTH: usize = 32;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObligationWire {
    node: NodeWire,
    #[serde(default)]
    model: Option<ModelWire>,
    ordinal: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelWire {
    identity: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NodeWire {
    package: String,
    path: Vec<SegmentWire>,
    ordinal: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
enum SegmentWire {
    Text(String),
    Node(NodeWire),
}

fn node_from_wire(wire: NodeWire, depth: usize) -> Result<NodeKey, String> {
    if depth > MAX_NODE_DEPTH {
        return Err("the obligation node nests past the supported depth".to_owned());
    }
    let path = wire
        .path
        .into_iter()
        .map(|segment| match segment {
            SegmentWire::Text(text) => Ok(PathSegment::Text(text)),
            SegmentWire::Node(node) => node_from_wire(node, depth + 1).map(PathSegment::Node),
        })
        .collect::<Result<Vec<_>, _>>()?;
    NodeKey::new(wire.package, path, wire.ordinal).map_err(|error| error.to_string())
}

/// Reads the wire form back through [`ObligationIdentity::new`].
pub(super) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<ObligationIdentity, D::Error> {
    let wire = ObligationWire::deserialize(deserializer)?;
    let node = node_from_wire(wire.node, 1).map_err(D::Error::custom)?;
    Ok(ObligationIdentity::new(
        node,
        wire.model.map(|model| ModelIdentity::new(model.identity)),
        wire.ordinal,
    ))
}

struct Node<'a> {
    key: &'a NodeKey,
    depth: usize,
}

struct Segments<'a> {
    key: &'a NodeKey,
    depth: usize,
}

impl Serialize for Segments<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let path = self.key.path();
        let mut sequence = serializer.serialize_seq(Some(path.len()))?;
        for segment in &path {
            match segment {
                PathSegment::Text(text) => {
                    sequence.serialize_element(&SegmentText { text })?;
                }
                PathSegment::Node(key) => sequence.serialize_element(&SegmentNode {
                    node: Node {
                        key,
                        depth: self.depth + 1,
                    },
                })?,
            }
        }
        sequence.end()
    }
}

#[derive(Serialize)]
struct SegmentText<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct SegmentNode<'a> {
    node: Node<'a>,
}

impl Serialize for Node<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.depth > MAX_NODE_DEPTH {
            return Err(S::Error::custom(
                "the obligation node nests past the supported depth",
            ));
        }
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("package", self.key.package())?;
        map.serialize_entry(
            "path",
            &Segments {
                key: self.key,
                depth: self.depth,
            },
        )?;
        map.serialize_entry("ordinal", &self.key.ordinal())?;
        map.end()
    }
}

#[derive(Serialize)]
struct ModelView<'a> {
    identity: &'a str,
}

/// Writes the wire form.
pub(super) fn serialize<S: Serializer>(
    obligation: &ObligationIdentity,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let model = obligation.model();
    let mut map = serializer.serialize_map(Some(2 + usize::from(model.is_some())))?;
    map.serialize_entry(
        "node",
        &Node {
            key: obligation.node(),
            depth: 1,
        },
    )?;
    if let Some(model) = model {
        map.serialize_entry(
            "model",
            &ModelView {
                identity: model.as_str(),
            },
        )?;
    }
    map.serialize_entry("ordinal", &obligation.ordinal())?;
    map.end()
}
