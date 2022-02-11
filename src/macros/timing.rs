/// Send your own timing metric in milliseconds
#[macro_export]
#[cfg(not(feature = "noop"))]
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
#[cfg(feature = "noop")]
macro_rules! timing {
    ($stat:literal, $ms:literal) => {};
    ($stat:path, $ms:literal) => {
        let _ = $stat;
    };
    ($stat:literal, $ms:literal; $( $key:expr => $value:expr ), *) => {
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $ms:literal; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
