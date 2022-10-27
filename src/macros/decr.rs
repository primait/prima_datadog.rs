/// Decrement a StatsD counter
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! decr {
    ($stat:expr) => {
        $crate::Datadog::decr($stat, &[]);
    };
    ($stat:path) => {
        $crate::Datadog::decr($stat.as_ref(), &[]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::decr($stat, &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::decr($stat.as_ref(), &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
