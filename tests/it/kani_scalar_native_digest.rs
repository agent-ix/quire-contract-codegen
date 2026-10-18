//! Independent JCS/SHA-256 oracle for the scalar proof-content string-only shape.
pub(crate) fn independent_digest(entries: &[(String, Vec<u8>)]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let artifacts = entries
        .iter()
        .map(|(path, bytes)| {
            format!(
                "{{\"contents\":{},\"path\":{}}}",
                serde_json::to_string(std::str::from_utf8(bytes).unwrap()).unwrap(),
                serde_json::to_string(path).unwrap()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let bytes = format!(
        "{{\"artifacts\":[{artifacts}],\"schema\":\"agent-ix/cg-kani-scalar-proof-content/v2\"}}"
    );
    Sha256::digest(bytes.as_bytes()).into()
}
