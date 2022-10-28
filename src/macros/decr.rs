/// Decrement a StatsD counter
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! decr {
    ($stat:expr) => {
        $crate::Datadog::decr($stat, $crate::EMPTY_TAGS);
    };
    ($stat:path) => {
        $crate::Datadog::decr($stat.as_ref(), $crate::EMPTY_TAGS);
    };
    ($stat:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::decr($stat, &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:path; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::decr($stat.as_ref(), &[$(::core::concat!($key, ":", $value)), *]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::decr($stat, &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::decr($stat.as_ref(), &[$(::core::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
