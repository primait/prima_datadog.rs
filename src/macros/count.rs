/// Make an arbitrary change to a StatsD counter
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! count {
    ($stat:expr, $count:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, vec![]);
        }
    };
    ($stat:path, $count:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, vec![]);
        }
    };
    ($stat:expr, $count:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $count:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! count {
    ($stat:expr, $count:expr) => {
        let _ = $stat;
        let _ = $count;
    };
    ($stat:path, $count:expr) => {
        let _ = $stat;
        let _ = $count;
    };
    ($stat:expr, $count:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $count;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $count:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $count;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
