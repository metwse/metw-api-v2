use crate::EPOCH;
use chrono::Utc;
use std::{sync::Mutex, time::Duration};

static LAST_OVERFLOW: Mutex<u64> = Mutex::new(0);
static INCREMENT_MAX: u64 = 2u64.pow(12);

static INCREMENT: Mutex<u64> = Mutex::new(0);

/// UUID generator inspired from Twitter's snowflake format.
///
/// | Field | Bits | Description |
/// | -- | -- | -- |
/// | Increment | 52 to 63 | For every ID that is generated, this number is incremented |
/// | Reserved for future use | 51 to 42 | |
/// | Timestamp | 41 to 0 | Milliseconds since metw.cc [`EPOCH`] |
pub fn snowflake() -> u64 {
    let timestamp = Utc::now().timestamp_millis() as u64 - *EPOCH;

    // Ensure the time is not yet May 15 2109 07:35:11
    assert!(timestamp < 2u64.pow(42) - 1);

    let mut increment = INCREMENT.lock().unwrap();
    let mut last_overflow = LAST_OVERFLOW.lock().unwrap();

    if *increment == 0 {
        *last_overflow = timestamp;
    }

    *increment += 1;

    if *increment == INCREMENT_MAX {
        *increment = 0;

        if *last_overflow == timestamp {
            std::thread::sleep(Duration::from_millis(1));
            return snowflake();
        }
    }

    timestamp | (*increment << 52)
}

#[cfg(test)]
#[test]
#[serial_test::serial]
fn test_snowflake() {
    use std::collections::HashSet;

    let cap = 2usize.pow(16);
    let mut snowflakes = HashSet::with_capacity(cap);

    for _ in 0..cap {
        let snowflake = snowflake();

        assert!(!snowflakes.contains(&snowflake));

        snowflakes.insert(snowflake);
    }
}
