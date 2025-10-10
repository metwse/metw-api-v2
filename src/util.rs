use std::time::{SystemTime, UNIX_EPOCH};

/// Unix timestamp as u64.
pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .expect("Greetings from 2025 to year 584 billion!")
}
