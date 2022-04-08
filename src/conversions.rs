//! This module contains methods for converting between time bases.
//!
//! Conversions are performed with the `checked_*` family of methods that will throw an error on
//! underflow or overflow instead of panicking or silent data corruption.

use crate::errors::Result;
use crate::EpochError;

/// nanoseconds per millisecond
pub const NS_PER_MS_U32: u32 = 1_000_000;

/// nanoseconds per microsecond
pub const NS_PER_US_U32: u32 = 1_000;

/// nanoseconds per millisecond
pub const NS_PER_MS_I128: i128 = 1_000_000;

/// nanoseconds per microsecond
pub const NS_PER_US_I128: i128 = 1_000;

/// Converts milliseconds to nanoseconds. Returns an error on underflow or overflow.
pub fn ms_to_ns_u32(ms: u32) -> Result<u32> {
    ms.checked_mul(NS_PER_MS_U32).ok_or_else(|| {
        EpochError::numeric_precision(format!("Conversion from {}ms to ns (u32)", ms).as_str())
    })
}

/// Converts microseconds to nanoseconds. Returns an error on underflow or overflow.
pub fn us_to_ns_u32(us: u32) -> Result<u32> {
    us.checked_mul(NS_PER_US_U32).ok_or_else(|| {
        EpochError::numeric_precision(format!("Conversion from {}us to ns (u32)", us).as_str())
    })
}

/// Converts milliseconds to nanoseconds. Returns an error on underflow or overflow.
pub fn ms_to_ns_i128(ms: i128) -> Result<i128> {
    ms.checked_mul(NS_PER_MS_I128).ok_or_else(|| {
        EpochError::numeric_precision(format!("Conversion from {}ms to ns (i128)", ms).as_str())
    })
}

/// Converts microseconds to nanoseconds. Returns an error on underflow or overflow.
pub fn us_to_ns_i128(us: i128) -> Result<i128> {
    us.checked_mul(NS_PER_US_I128).ok_or_else(|| {
        EpochError::numeric_precision(format!("Conversion from {}us to ns (i128)", us).as_str())
    })
}

/// Converts nanoseconds to milliseconds. Returns an error on underflow or overflow.
pub fn ns_to_ms_i128(ns: i128) -> Result<i128> {
    ns.checked_div(NS_PER_MS_I128).ok_or_else(|| {
        EpochError::numeric_precision(format!("Conversion from {}ns to ms (i128)", ns).as_str())
    })
}

/// Converts nanoseconds to microseconds. Returns an error on underflow or overflow.
pub fn ns_to_us_i128(ns: i128) -> Result<i128> {
    ns.checked_div(NS_PER_US_I128).ok_or_else(|| {
        EpochError::numeric_precision(format!("Conversion from {}ns to us (i128)", ns).as_str())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ms_to_ns_u32_zero() {
        assert_eq!(ms_to_ns_u32(0).unwrap(), 0)
    }

    #[test]
    fn test_ms_to_ns_u32_one() {
        assert_eq!(ms_to_ns_u32(1).unwrap(), 1_000_000)
    }

    #[test]
    fn test_ms_to_ns_u32_bad() {
        assert!(ms_to_ns_u32(u32::MAX).is_err());
    }

    #[test]
    fn test_ms_to_ns_i128_zero() {
        assert_eq!(ms_to_ns_i128(0).unwrap(), 0)
    }

    #[test]
    fn test_ms_to_ns_i128_one() {
        assert_eq!(ms_to_ns_i128(1).unwrap(), 1_000_000)
    }

    #[test]
    fn test_ms_to_ns_i128_minus_one() {
        assert_eq!(ms_to_ns_i128(-1).unwrap(), -1_000_000)
    }

    #[test]
    fn test_us_to_ns_u32_zero() {
        assert_eq!(us_to_ns_u32(0).unwrap(), 0)
    }

    #[test]
    fn test_us_to_ns_u32_one() {
        assert_eq!(us_to_ns_u32(1).unwrap(), 1_000)
    }

    #[test]
    fn test_us_to_ns_i128_zero() {
        assert_eq!(us_to_ns_i128(0).unwrap(), 0)
    }

    #[test]
    fn test_us_to_ns_i128_one() {
        assert_eq!(us_to_ns_i128(1).unwrap(), 1_000)
    }

    #[test]
    fn test_us_to_ns_i128_minus_one() {
        assert_eq!(us_to_ns_i128(-1).unwrap(), -1_000)
    }
}
