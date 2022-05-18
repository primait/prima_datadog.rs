/// Report a value in a set
#[macro_export]
macro_rules! set {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::global().set($stat, $val, vec![]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::global().set($stat.as_ref(), $val, vec![]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().set($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::global().set($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
