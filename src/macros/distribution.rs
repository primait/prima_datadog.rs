/// Report a value in a distribution
/// NOTE: avoid high tag cardinality. See note in lib docs!
#[macro_export]
macro_rules! distribution {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::distribution($stat, $val, vec![]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::distribution($stat.as_ref(), $val, vec![]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::distribution($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::distribution($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
