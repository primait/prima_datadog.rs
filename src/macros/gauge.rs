/// Report an arbitrary value as a gauge
/// NOTE: avoid high tag cardinality. See note in lib docs!
#[macro_export]
macro_rules! gauge {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::gauge($stat, $val, vec![]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::gauge($stat.as_ref(), $val, vec![]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::gauge($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::gauge($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
