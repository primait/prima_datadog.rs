/// Send your own timing metric in milliseconds
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! timing {
    ($stat:expr, $ms:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat, $ms, vec![]);
        }
    };
    ($stat:path, $ms:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat.as_ref(), $ms, vec![]);
        }
    };
    ($stat:expr, $ms:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat, $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $ms:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat.as_ref(), $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! timing {
    ($stat:expr, $ms:expr) => {
        let _ = $stat;
        let _ = $ms;
    };
    ($stat:path, $ms:expr) => {
        let _ = $stat;
        let _ = $ms;
    };
    ($stat:expr, $ms:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $ms;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $ms:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $ms;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
