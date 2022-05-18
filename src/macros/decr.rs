/// Decrement a StatsD counter
#[macro_export]
macro_rules! decr {
    ($stat:expr) => {
        $crate::Datadog::global().decr($stat, vec![]);
    };
    ($stat:path) => {
        $crate::Datadog::global().decr($stat.as_ref(), vec![]);
    };
    ($stat:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().decr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().decr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
