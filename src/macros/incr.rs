/// Increment a StatsD counter
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! incr {
    ($stat:expr) => {
        $crate::Datadog::incr($stat, &[]);
    };
    ($stat:path) => {
        $crate::Datadog::incr($stat.as_ref(), &[]);
    };
    ($stat:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::incr($stat, &[$(std::concat!($key, ":", $value)), *]);
    };
    ($stat:path; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::incr($stat.as_ref(), &[$(std::concat!($key, ":", $value)), *]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::incr($stat, &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::incr($stat.as_ref(), &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
