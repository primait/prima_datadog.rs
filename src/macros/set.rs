/// Report a value in a set
#[macro_export]
#[cfg(not(feature = "dev-null"))]
macro_rules! set {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

#[macro_export]
#[cfg(feature = "dev-null")]
macro_rules! set {
    // Keep all these pattern in order to avoid warning generation in the projects that use this lib
    // at compile time
    ($stat:literal, $val:literal) => {};
    ($stat:path, $val:literal) => {};
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {};
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {};
}
