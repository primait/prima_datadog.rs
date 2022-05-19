/// Decrement a StatsD counter
#[macro_export]
macro_rules! decr {
    ($stat:expr) => {
        $crate::Datadog::decr($stat, vec![]);
    };
    ($stat:path) => {
        $crate::Datadog::decr($stat.as_ref(), vec![]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::decr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::decr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
