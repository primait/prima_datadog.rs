/// Report an arbitrary value as a gauge
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! gauge {
    ($stat:expr, $val:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
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
#[cfg(feature = "noop")]
macro_rules! gauge {
    ($stat:expr, $val:expr) => {
        let _ = $stat;
        let _ = $val;
    };
    ($stat:path, $val:expr) => {
        let _ = $stat;
        let _ = $val;
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $val;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $val;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
