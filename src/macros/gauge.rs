/// Report an arbitrary value as a gauge
#[macro_export]
#[cfg(not(feature = "dev-null"))]
macro_rules! gauge {
    ($stat:literal, $val:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

#[macro_export]
#[cfg(feature = "dev-null")]
macro_rules! gauge {
    // Keep all these pattern in order to avoid warning generation in the projects that use this lib
    // at compile time
    ($stat:literal, $val:expr) => {};
    ($stat:path, $val:expr) => {};
    ($stat:literal, $val:expr; $( $key:expr => $value:expr ), *) => {};
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {};
}
