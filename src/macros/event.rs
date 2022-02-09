/// Send a custom event as a title and a body
#[macro_export]
macro_rules! event {
    ($stat:path, $text:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat.as_ref(), $text, vec![]);
        }
    };
    ($stat:expr, $text:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat, $text, vec![]);
        }
    };
    ($stat:expr, $text:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat, $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $text:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat.as_ref(), $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
