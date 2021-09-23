/// Report a value in a distribution
#[macro_export]
macro_rules! distribution {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
