/// Send your own timing metric in milliseconds
#[macro_export]
#[cfg(not(feature = "dev-null"))]
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

#[macro_export]
#[cfg(feature = "dev-null")]
macro_rules! timing {
    // Keep all these pattern in order to avoid warning generation in the projects that use this lib
    // at compile time
    ($stat:literal, $ms:literal) => {};
    ($stat:path, $ms:literal) => {};
    ($stat:literal, $ms:literal; $( $key:expr => $value:expr ), *) => {};
    ($stat:path, $ms:literal; $( $key:expr => $value:expr ), *) => {};
}
