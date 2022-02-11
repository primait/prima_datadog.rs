/// Make an arbitrary change to a StatsD counter
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! count {
    ($stat:literal, $count:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, vec![]);
        }
    };
    ($stat:path, $count:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, vec![]);
        }
    };
    ($stat:literal, $count:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $count:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! count {
    ($stat:literal, $count:literal) => {};
    ($stat:path, $count:literal) => {
        let _ = $stat;
    };
    ($stat:literal, $count:literal; $( $key:expr => $value:expr ), *) => {
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $count:literal; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
