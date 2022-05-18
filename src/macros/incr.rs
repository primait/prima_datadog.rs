/// Increment a StatsD counter
#[macro_export]
macro_rules! incr {
    ($stat:expr) => {
        $crate::Datadog::global().incr($stat, vec![]);
    };
    ($stat:path) => {
        $crate::Datadog::global().incr($stat.as_ref(), vec![]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().incr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().incr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
