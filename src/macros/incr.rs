/// Increment a StatsD counter
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! incr {
    ($stat:expr) => {
        $crate::Datadog::incr($stat, $crate::EMPTY_TAGS);
    };
    ($stat:path) => {
        $crate::Datadog::incr($stat.as_ref(), $crate::EMPTY_TAGS);
    };
    ($stat:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::incr($stat, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:path; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::incr($stat.as_ref(), &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::incr($stat, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::incr($stat.as_ref(), &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
