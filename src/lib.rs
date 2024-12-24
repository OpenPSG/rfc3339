//! # Unix Timestamp to RFC3339 Converter
//!
//! This library provides functionality to convert Unix timestamps into
//! RFC3339 formatted date-time strings, specifically in the UTC timezone.
//! It's designed to work both with and without the standard library (`no_std`).
//!
//! ## Features
//! - No standard library dependency when built with default features disabled.
//! - Supports heapless operation for embedded environments.
//! 
//! ## Usage
//! 
//! ```rust
//! use rfc3339::format_unix;
//! 
//! let timestamp = format_unix(1609459200, 0);
//! assert_eq!(timestamp, "2021-01-01T00:00:00.000000Z");
//! ```
//! 
//! ## References
//! 
//! - [RFC3339](https://tools.ietf.org/html/rfc3339)
//! - [Unix Timestamp](https://en.wikipedia.org/wiki/Unix_time)
//! - [Rata Die](https://en.wikipedia.org/wiki/Rata_Die)
//! 
//! ## License
//! 
//! Licensed under the Mozilla Public License, v. 2.0, see LICENSE for details.

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt::Write;

#[cfg(not(feature = "std"))]
use heapless::String;

const SECONDS_PER_DAY: u64 = 86400;
const DAY_OFFSETS: [u64; 13] = [0, 306, 337, 0, 31, 61, 92, 122, 153, 184, 214, 245, 275];
// Unix epoch in seconds (Gregorian calendar).
const UNIX_EPOCH: u64 = 62135683200;

/// A timestamp in RFC3339 format.
#[cfg(feature = "std")]
pub type Timestamp = String;
#[cfg(not(feature = "std"))]
pub type Timestamp = String<27>;

/// Converts a Unix timestamp into an RFC3339 formatted date-time string in UTC.
///
/// # Arguments
///
/// * `seconds` - The number of seconds since Unix Epoch.
/// * `micros` - Microseconds part to be included in the timestamp.
///
/// # Examples
///
/// ```rust
/// use rfc3339::format_unix;
/// 
/// let timestamp = format_unix(1609459200, 0);
/// assert_eq!(timestamp, "2021-01-01T00:00:00.000000Z");
/// ```
pub fn format_unix(seconds: u64, micros: u32) -> Timestamp {
    let days_since_epoch = (seconds + UNIX_EPOCH) / SECONDS_PER_DAY;
    let (year, month, day) = rdn_to_ymd(days_since_epoch);
    let sec = (seconds + UNIX_EPOCH) % SECONDS_PER_DAY;
    let hour = sec / 3600;
    let minute = (sec % 3600) / 60;
    let second = sec % 60;

    let mut output = Timestamp::new();
    let _ = write!(
        output,
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}Z",
        year, month, day, hour, minute, second, micros
    );
    output
}

/// Rata Die algorithm by Peter Baum.
fn rdn_to_ymd(rdn: u64) -> (u32, u32, u32) {
    let z = rdn + 306;
    let h = 100 * z - 25;
    let a = h / 3652425;
    let b = a - (a >> 2);
    let mut y = (100 * b + h) / 36525;
    let d = b + z - (1461 * y >> 2);
    let mut m = (535 * d + 48950) >> 14;

    if m > 12 {
        y += 1;
        m -= 12;
    }

    (y as u32, m as u32, (d - DAY_OFFSETS[m as usize]) as u32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_unix() {
        let seconds: u64 = 1445470140;
        let micros: u32 = 123456;

        let result = format_unix(seconds, micros);

        let expected = "2015-10-21T23:29:00.123456Z";
        assert_eq!(result.as_str(), expected);
    }
}