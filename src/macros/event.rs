/// Send a custom event as a title and a body
#[macro_export]
#[cfg(not(feature = "noop"))]
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
    ($stat:literal, $text:literal; $( $key:expr => $value:expr ), *) => {
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

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! event {
    ($stat:literal, $text:literal) => {};
    ($stat:path, $text:literal) => {
        let _ = $stat;
    };
    ($stat:literal, $text:literal; $( $key:expr => $value:expr ), *) => {
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $text:literal; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
