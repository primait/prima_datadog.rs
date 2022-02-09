/// Make an arbitrary change to a StatsD counter
#[macro_export]
macro_rules! count {
    ($stat:expr, $count:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, vec![]);
        }
    };
    ($stat:path, $count:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, vec![]);
        }
    };
    ($stat:expr, $count:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $count:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
