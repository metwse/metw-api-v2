use serde::Serializer;
use sqlx::types::BitVec;

/// Serializes [`BitVec`] as hex string.
pub fn bitvec_as_hex<S: Serializer>(val: &BitVec, serializer: S) -> Result<S::Ok, S::Error> {
    let bytes = val.to_bytes();
    let hex_string = hex::encode(bytes);

    serializer.serialize_str(&hex_string)
}
