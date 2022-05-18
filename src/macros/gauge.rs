/// Report an arbitrary value as a gauge
#[macro_export]
macro_rules! gauge {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::global().gauge($stat, $val, vec![]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::global().gauge($stat.as_ref(), $val, vec![]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().gauge($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().gauge($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
