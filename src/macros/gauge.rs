/// Report an arbitrary value as a gauge
#[macro_export]
macro_rules! gauge {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
