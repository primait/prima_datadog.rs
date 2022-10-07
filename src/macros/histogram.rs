/// Report a value in a histogram
/// NOTE: avoid high tag cardinality. See note in lib docs!
#[macro_export]
macro_rules! histogram {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::histogram($stat, $val, vec![]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::histogram($stat.as_ref(), $val, vec![]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::histogram($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::histogram($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
