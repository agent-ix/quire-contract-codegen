//! Bounded, iterative reconstruction of equality operand types.

use super::{
    aggregate_members, bound_members, is_option_node, literal_count, lookup, node_key,
    read_record_field, read_reference, resolve_scalar, CardinalityBound, CheckedNodeId,
    CheckedNodeTag, CheckedSemanticNodeV2, CollectionKind, CollectionType, CompositeDeclaration,
    CompositeEqualityRefusal, CompositeShape, FieldDeclaration, Graph, NodeKey, Presence,
    TypeClosure, ValueType, COLLECTION_BOUNDS_MEMBERS,
    COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
};
use std::collections::{BTreeMap, BTreeSet};

enum Frame {
    Enter {
        id: CheckedNodeId,
        external_bound: Option<CheckedNodeId>,
    },
    FinishBound(CheckedNodeId),
    FinishOption(CheckedNodeId),
    FinishCollection {
        id: CheckedNodeId,
        kind: CollectionKind,
        cardinality: CardinalityBound,
    },
    FinishRecord {
        id: CheckedNodeId,
        key: NodeKey,
        fields: Vec<(String, bool)>,
    },
    FinishTuple {
        id: CheckedNodeId,
        key: NodeKey,
        positions: usize,
    },
}

impl TypeClosure {
    fn charge(&mut self) -> Result<(), CompositeEqualityRefusal> {
        self.work_consumed = self.work_consumed.saturating_add(1);
        if self.work_consumed > COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT {
            return Err(CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: COMPOSITE_EQUALITY_TYPE_RESOLUTION_WORK_LIMIT,
                consumed: self.work_consumed,
            });
        }
        Ok(())
    }
}

fn entered(
    active: &mut BTreeSet<CheckedNodeId>,
    id: &CheckedNodeId,
) -> Result<(), CompositeEqualityRefusal> {
    if !active.insert(id.clone()) {
        return Err(CompositeEqualityRefusal::TypeResolutionCycle {
            repeated_type_node_id: id.clone(),
        });
    }
    Ok(())
}

fn collection_parts(
    graph: &Graph<'_>,
    closure: &mut TypeClosure,
    id: &CheckedNodeId,
    node: &CheckedSemanticNodeV2,
    external_bound: Option<&CheckedSemanticNodeV2>,
) -> Result<(CheckedNodeId, CollectionKind, CardinalityBound), CompositeEqualityRefusal> {
    let malformed = || CompositeEqualityRefusal::MalformedComposite {
        composite: id.clone(),
    };
    let members = aggregate_members(&node.body).ok_or_else(malformed)?;
    let (element_term, bound) = match (members, external_bound) {
        ([element], Some(bound)) if &*node.semantic_form == "sequence" => (element, bound),
        ([element, bound], None) => {
            closure.charge()?; // charge the followed bound before reading or descending
            let target = read_reference(bound).ok_or_else(malformed)?;
            (element, lookup(graph, &target)?)
        }
        _ => return Err(malformed()),
    };
    let element = read_reference(element_term).ok_or_else(malformed)?;
    if CheckedNodeTag::from_wire(&bound.node_tag) != Some(CheckedNodeTag::BoundedDomain)
        || &*bound.semantic_form != "collection_bounds"
    {
        return Err(CompositeEqualityRefusal::UnreadableBound {
            bound: bound.node_id.clone(),
        });
    }
    let unreadable = || CompositeEqualityRefusal::UnreadableBound {
        bound: bound.node_id.clone(),
    };
    let terms = aggregate_members(&bound.body).ok_or_else(unreadable)?;
    let [minimum, maximum] =
        bound_members(terms, COLLECTION_BOUNDS_MEMBERS).ok_or_else(unreadable)?;
    let minimum = literal_count(minimum).ok_or_else(unreadable)?;
    let maximum = literal_count(maximum).ok_or_else(unreadable)?;
    let cardinality = CardinalityBound::new(minimum, maximum).map_err(|_| unreadable())?;
    let kind = match &*node.semantic_form {
        "sequence" => CollectionKind::Sequence,
        "set" => CollectionKind::Set,
        "bag" => CollectionKind::Bag,
        "ordered_set" => CollectionKind::OrderedSet,
        _ => return Err(malformed()),
    };
    Ok((element, kind, cardinality))
}

/// Resolve a root within the item's shared closure and work counter.
pub(super) fn resolve_type(
    graph: &Graph<'_>,
    bounds_by_type: &BTreeMap<&CheckedNodeId, Vec<&CheckedSemanticNodeV2>>,
    closure: &mut TypeClosure,
    type_id: &CheckedNodeId,
) -> Result<ValueType, CompositeEqualityRefusal> {
    let mut frames = vec![Frame::Enter {
        id: type_id.clone(),
        external_bound: None,
    }];
    let mut values = Vec::new();
    let mut active = BTreeSet::new();
    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Enter { id, external_bound } => {
                closure.charge()?;
                let node = lookup(graph, &id)?;
                match CheckedNodeTag::from_wire(&node.node_tag) {
                    Some(CheckedNodeTag::ScalarType) => {
                        let bounds = bounds_by_type.get(&id).map_or(&[][..], Vec::as_slice);
                        values.push(resolve_scalar(bounds, &id, node)?);
                    }
                    Some(CheckedNodeTag::BoundedDomain) => {
                        let base = lookup(graph, &node.semantic_type)?;
                        let unsupported = || CompositeEqualityRefusal::Unsupported {
                            unsupported_node_id: id.clone(),
                            node_tag: "bounded_domain",
                        };
                        if &*node.semantic_form == "collection_bounds" {
                            if CheckedNodeTag::from_wire(&base.node_tag)
                                != Some(CheckedNodeTag::CompositeType)
                                || &*base.semantic_form != "sequence"
                            {
                                return Err(unsupported());
                            }
                            entered(&mut active, &id)?;
                            frames.push(Frame::FinishBound(id.clone()));
                            frames.push(Frame::Enter {
                                id: base.node_id.clone(),
                                external_bound: Some(id),
                            });
                        } else {
                            closure.charge()?; // the bounded-domain semantic_type edge
                            if CheckedNodeTag::from_wire(&base.node_tag)
                                != Some(CheckedNodeTag::ScalarType)
                            {
                                return Err(unsupported());
                            }
                            let expected_form = match &*base.semantic_form {
                                "integer" => "integer_range",
                                "rational" => "rational_range",
                                "decimal" => "decimal_range",
                                "text" => "text_bounds",
                                "float32" | "float64" => "float_rounding",
                                _ => return Err(unsupported()),
                            };
                            if &*node.semantic_form != expected_form {
                                return Err(CompositeEqualityRefusal::MissingBound {
                                    bounded_type: id,
                                    expected_form,
                                });
                            }
                            values.push(resolve_scalar(core::slice::from_ref(&node), &id, base)?);
                        }
                    }
                    Some(CheckedNodeTag::CompositeType) => match &*node.semantic_form {
                        "record" | "tuple" => {
                            let key = node_key(&id)?;
                            if closure.composites.contains_key(&key)
                                || closure.in_progress.contains(&key)
                            {
                                values.push(ValueType::Composite(key));
                                continue;
                            }
                            let members = aggregate_members(&node.body).ok_or_else(|| {
                                CompositeEqualityRefusal::MalformedComposite {
                                    composite: id.clone(),
                                }
                            })?;
                            let malformed = || CompositeEqualityRefusal::MalformedComposite {
                                composite: id.clone(),
                            };
                            let mut children = Vec::with_capacity(members.len());
                            if &*node.semantic_form == "record" {
                                let mut fields = Vec::with_capacity(members.len());
                                for member in members {
                                    let (name, target, wrapped) =
                                        read_record_field(member).ok_or_else(malformed)?;
                                    if wrapped && !is_option_node(lookup(graph, &target)?) {
                                        return Err(malformed());
                                    }
                                    fields.push((name, wrapped));
                                    children.push(target);
                                }
                                closure.in_progress.insert(key);
                                frames.push(Frame::FinishRecord { id, key, fields });
                            } else {
                                for member in members {
                                    children.push(read_reference(member).ok_or_else(malformed)?);
                                }
                                closure.in_progress.insert(key);
                                frames.push(Frame::FinishTuple {
                                    id,
                                    key,
                                    positions: children.len(),
                                });
                            }
                            for child in children.into_iter().rev() {
                                frames.push(Frame::Enter {
                                    id: child,
                                    external_bound: None,
                                });
                            }
                        }
                        "option" => {
                            entered(&mut active, &id)?;
                            let members = aggregate_members(&node.body).ok_or_else(|| {
                                CompositeEqualityRefusal::MalformedComposite {
                                    composite: id.clone(),
                                }
                            })?;
                            let [payload] = members else {
                                return Err(CompositeEqualityRefusal::MalformedComposite {
                                    composite: id,
                                });
                            };
                            let target = read_reference(payload).ok_or_else(|| {
                                CompositeEqualityRefusal::MalformedComposite {
                                    composite: id.clone(),
                                }
                            })?;
                            frames.push(Frame::FinishOption(id));
                            frames.push(Frame::Enter {
                                id: target,
                                external_bound: None,
                            });
                        }
                        "sequence" | "set" | "bag" | "ordered_set" => {
                            entered(&mut active, &id)?;
                            let bound = external_bound
                                .as_ref()
                                .map(|target| lookup(graph, target))
                                .transpose()?;
                            let (element, kind, cardinality) =
                                collection_parts(graph, closure, &id, node, bound)?;
                            frames.push(Frame::FinishCollection {
                                id,
                                kind,
                                cardinality,
                            });
                            frames.push(Frame::Enter {
                                id: element,
                                external_bound: None,
                            });
                        }
                        "reference" => {
                            return Err(CompositeEqualityRefusal::BlockedOnUpstream {
                                unsupported_node_id: id,
                                node_tag: "composite_type.reference",
                                issue: super::UpstreamBlocker::QuireSpecLanguage120,
                            })
                        }
                        _ => {
                            return Err(CompositeEqualityRefusal::Unsupported {
                                unsupported_node_id: id,
                                node_tag: "composite_type",
                            })
                        }
                    },
                    _ => {
                        return Err(CompositeEqualityRefusal::Unsupported {
                            unsupported_node_id: id,
                            node_tag: "unknown",
                        })
                    }
                }
            }
            Frame::FinishBound(id) => {
                active.remove(&id);
            }
            Frame::FinishOption(id) => {
                let payload =
                    values
                        .pop()
                        .ok_or_else(|| CompositeEqualityRefusal::MalformedComposite {
                            composite: id.clone(),
                        })?;
                values.push(ValueType::option(payload));
                active.remove(&id);
            }
            Frame::FinishCollection {
                id,
                kind,
                cardinality,
            } => {
                let element =
                    values
                        .pop()
                        .ok_or_else(|| CompositeEqualityRefusal::MalformedComposite {
                            composite: id.clone(),
                        })?;
                values.push(ValueType::collection(CollectionType::new(
                    kind,
                    element,
                    cardinality,
                )));
                active.remove(&id);
            }
            Frame::FinishRecord { id, key, fields } => {
                let mut built = Vec::with_capacity(fields.len());
                for (name, wrapped) in fields.into_iter().rev() {
                    let mut value = values.pop().ok_or_else(|| {
                        CompositeEqualityRefusal::MalformedComposite {
                            composite: id.clone(),
                        }
                    })?;
                    let (value, presence) = if wrapped {
                        let ValueType::Option(payload) = &mut value else {
                            return Err(CompositeEqualityRefusal::MalformedComposite {
                                composite: id,
                            });
                        };
                        // `ValueType` owns iterative Drop glue, so detach its child before
                        // the discarded Option shell is dropped.
                        (
                            std::mem::replace(payload.as_mut(), ValueType::Boolean),
                            Presence::Optional,
                        )
                    } else {
                        (value, Presence::Required)
                    };
                    built.push(FieldDeclaration::new(name, value, presence));
                }
                built.reverse();
                closure.in_progress.remove(&key);
                closure.composites.insert(
                    key,
                    CompositeDeclaration::new(
                        key,
                        id.digest.to_string(),
                        CompositeShape::Record(built),
                    ),
                );
                closure.node_ids.push(id);
                values.push(ValueType::Composite(key));
            }
            Frame::FinishTuple { id, key, positions } => {
                let mut built = Vec::with_capacity(positions);
                for _ in 0..positions {
                    built.push(values.pop().ok_or_else(|| {
                        CompositeEqualityRefusal::MalformedComposite {
                            composite: id.clone(),
                        }
                    })?);
                }
                built.reverse();
                closure.in_progress.remove(&key);
                closure.composites.insert(
                    key,
                    CompositeDeclaration::new(
                        key,
                        id.digest.to_string(),
                        CompositeShape::Tuple(built),
                    ),
                );
                closure.node_ids.push(id);
                values.push(ValueType::Composite(key));
            }
        }
    }
    values
        .pop()
        .ok_or_else(|| CompositeEqualityRefusal::UnknownTypeNode {
            type_node_id: type_id.clone(),
        })
}
