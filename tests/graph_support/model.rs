//! The shared authored domain document used by collection and graph QSL fixtures.
use serde_json::{json, Value};
const NODE_ID: &str = "ix://ir268.test/domain/Node";
const SMALL_ID: &str = "ix://ir268.test/domain/SmallInt";
const POPULATION_ID: &str = "ix://ir268.test/domain/CollectionNodes";
const SOURCE_ID: &str = "ix://ir268.test/spec";
const ZERO_SHA256: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";

pub(crate) fn model_document() -> Value {
    let field = |name: &str,
                 type_ref: &str,
                 lower: u64,
                 upper: u64,
                 ordered: bool,
                 unique: bool,
                 nullable: bool,
                 presence: &str| {
        json!({
            "identity": format!("{NODE_ID}/{name}"),
            "name": name,
            "typeRef": type_ref,
            "presence": presence,
            "nullable": nullable,
            "defaultKind": "none",
            "origin": {"generated": {
                "generatorIdentity": SOURCE_ID,
                "generatorVersion": "1.0.0",
                "inputIdentities": [format!("{NODE_ID}/{name}")]
            }},
            "multiplicity": {
                "lower": lower,
                "upper": upper,
                "ordered": ordered,
                "unique": unique
            }
        })
    };
    json!({
        "contractVersion": "2.0.0",
        "canonicalization": {"algorithm": "rfc8785-v1", "digest": "sha256-jcs"},
        "source": {
            "identity": SOURCE_ID,
            "version": "1.0.0",
            "dialect": "spec-bundle",
            "digest": ZERO_SHA256
        },
        "package": {
            "identity": "ir268.test/domain", "version": "1.0.0",
            "manifestDigest": ZERO_SHA256,
            "mappingVersions": [], "profileVersions": [], "lockDigest": ZERO_SHA256
        },
        "constructs": [
            {"kind": {"module": "ir268.test/domain", "name": "entity"},
             "moduleVersion": "1.0.0", "manifestDigest": ZERO_SHA256,
             "construct": {"identity": "none", "shape": "record", "members": {},
                            "meaning": "quire.meaning.model.object-type/v1"}},
            {"kind": {"module": "ir268.test/domain", "name": "value_type"},
             "moduleVersion": "1.0.0", "manifestDigest": ZERO_SHA256,
             "construct": {"identity": "none", "shape": "record", "members": {},
                            "meaning": "quire.meaning.model.value-type/v1"}},
            {"kind": {"module": "ir268.test/domain", "name": "population"},
             "moduleVersion": "1.0.0", "manifestDigest": ZERO_SHA256,
             "construct": {"identity": "none", "shape": "record", "members": {},
                            "meaning": "quire.meaning.model.population/v1"}}
        ],
        "occurrences": [], "extensions": [],
        "types": [
            {
                "identity": SMALL_ID,
                "displayName": "SmallInt",
                "kind": {"module": "ir268.test/domain", "name": "value_type"},
                "roles": [],
                "origin": {"generated": {
                    "generatorIdentity": SOURCE_ID,
                    "generatorVersion": "1.0.0",
                    "inputIdentities": [SMALL_ID]
                }},
                "extensions": [], "unknownPolicy": "reject",
                "scalar": "integer",
                "constraints": [
                    {
                        "identity": "ix://ir268.test/domain/SmallInt/min",
                        "keyword": "min", "operands": {"value": "0"},
                        "appliesTo": "ix://quire/native/Integer", "diagnosticCode": "ir268.minimum",
                        "origin": {"generated": {
                            "generatorIdentity": SOURCE_ID,
                            "generatorVersion": "1.0.0",
                            "inputIdentities": [SMALL_ID]
                        }}
                    },
                    {
                        "identity": "ix://ir268.test/domain/SmallInt/max",
                        "keyword": "max", "operands": {"value": "3"},
                        "appliesTo": "ix://quire/native/Integer", "diagnosticCode": "ir268.maximum",
                        "origin": {"generated": {
                            "generatorIdentity": SOURCE_ID,
                            "generatorVersion": "1.0.0",
                            "inputIdentities": [SMALL_ID]
                        }}
                    }
                ]
            },
            {
                "identity": NODE_ID,
                "displayName": "Node",
                "kind": {"module": "ir268.test/domain", "name": "entity"},
                "roles": [],
                "origin": {"generated": {
                    "generatorIdentity": SOURCE_ID,
                    "generatorVersion": "1.0.0",
                    "inputIdentities": [NODE_ID]
                }},
                "constraints": [], "extensions": [],
                "unknownPolicy": "reject", "supertypes": [],
                "fields": [
                    field("items", SMALL_ID, 0, 3, true, false, false, "required"),
                    field("flags", "ix://quire/native/Boolean", 0, 2, true, false, false, "required"),
                    field("next", NODE_ID, 1, 1, false, true, false, "required"),
                    field("maybe", NODE_ID, 1, 1, false, true, false, "optional"),
                    field("many", NODE_ID, 0, 2, true, false, false, "required")
                ],
                "operations": []
            }
        ],
        "populations": [{
            "identity": POPULATION_ID,
            "displayName": "CollectionNodes",
            "kind": {"module": "ir268.test/domain", "name": "population"},
            "members": [NODE_ID],
            "extent": "closed",
            "origin": {"generated": {
                "generatorIdentity": SOURCE_ID,
                "generatorVersion": "1.0.0",
                "inputIdentities": [POPULATION_ID]
            }}
        }]
    })
}

