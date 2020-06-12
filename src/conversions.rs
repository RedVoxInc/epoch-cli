const NS_PER_US: f64 = 1_000.0;
const NS_PER_MS: f64 = 1_000_000.0;

#[inline]
pub fn us_to_ns(us: f64) -> f64 {
    us * NS_PER_US
}

#[inline]
pub fn ms_to_ns(ms: f64) -> f64 {
    ms * NS_PER_MS
}

#[inline]
pub fn ns_to_us(ns: f64) -> f64 {
    ns / NS_PER_US
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_us_to_ns() {
        assert_eq!(us_to_ns(1.0), 1_000.0)
    }

    #[test]
    fn test_ms_to_ns() {
        assert_eq!(ms_to_ns(1.0), 1_000_000.0)
    }

    #[test]
    fn test_ns_to_us() {
        assert_eq!(ns_to_us(1_000.0), 1.0)
    }
}
