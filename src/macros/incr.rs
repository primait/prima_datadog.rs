/// Increment a StatsD counter
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! incr {
    ($stat:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat, vec![]);
        }
    };
    ($stat:path) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat.as_ref(), vec![]);
        }
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! incr {
    ($stat:literal) => {};
    ($stat:path) => {
        let _ = $stat;
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
