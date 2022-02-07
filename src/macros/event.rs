/// Send a custom event as a title and a body
#[macro_export]
macro_rules! event {
    ($stat:literal, $text:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat, $text, vec![]);
        }
    };
    ($stat:path, $text:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat.as_ref(), $text, vec![]);
        }
    };
    ($stat:literal, $text:ident) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat, $text, vec![]);
        }
    };
    ($stat:literal, $text:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat, $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:literal, $text:ident; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat, $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $text:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().event($stat.as_ref(), $text, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
