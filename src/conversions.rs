use crate::errors::Result;
use crate::EpochError;

pub const NS_PER_MS_U32: u32 = 1_000_000;
pub const NS_PER_US_U32: u32 = 1_000;
pub const NS_PER_MS_I128: i128 = 1_000_000;
pub const NS_PER_US_I128: i128 = 1_000;

pub fn ms_to_ns_u32(ms: u32) -> Result<u32> {
    ms.checked_mul(NS_PER_MS_U32)
        .ok_or_else(|| EpochError::numeric_precision())
}

pub fn us_to_ns_u32(us: u32) -> Result<u32> {
    us.checked_mul(NS_PER_US_U32)
        .ok_or_else(|| EpochError::numeric_precision())
}

pub fn ms_to_ns_i128(ms: i128) -> Result<i128> {
    ms.checked_mul(NS_PER_MS_I128)
        .ok_or_else(|| EpochError::numeric_precision())
}

pub fn us_to_ns_i128(us: i128) -> Result<i128> {
    us.checked_mul(NS_PER_US_I128)
        .ok_or_else(|| EpochError::numeric_precision())
}

pub fn ns_to_ms_i128(ns: i128) -> Result<i128> {
    ns.checked_div(NS_PER_MS_I128)
        .ok_or_else(|| EpochError::numeric_precision())
}

pub fn ns_to_us_i128(ns: i128) -> Result<i128> {
    ns.checked_div(NS_PER_US_I128)
        .ok_or_else(|| EpochError::numeric_precision())
}
