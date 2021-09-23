/// Send your own timing metric in milliseconds
#[macro_export]
macro_rules! timing {
    ($stat:literal, $ms:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat, $ms, vec![]);
        }
    };
    ($stat:path, $ms:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat.as_ref(), $ms, vec![]);
        }
    };
    ($stat:literal, $ms:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat, $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $ms:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat.as_ref(), $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
