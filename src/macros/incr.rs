/// Increment a StatsD counter
/// NOTE: avoid high tag cardinality. See note in lib docs!
#[macro_export]
macro_rules! incr {
    ($stat:expr) => {
        $crate::Datadog::incr($stat, vec![]);
    };
    ($stat:path) => {
        $crate::Datadog::incr($stat.as_ref(), vec![]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::incr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::incr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
