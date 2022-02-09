/// Increment a StatsD counter
#[macro_export]
#[cfg(not(feature = "dev-null"))]
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
#[cfg(feature = "dev-null")]
macro_rules! incr {
    // Keep all these pattern in order to avoid warning generation in the projects that use this lib
    // at compile time
    ($stat:literal) => {};
    ($stat:path) => {};
    ($stat:literal; $( $key:expr => $value:expr ), *) => {};
    ($stat:path; $( $key:expr => $value:expr ), *) => {};
}
