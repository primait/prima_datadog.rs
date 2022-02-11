/// Time a block of code (reports in ms)
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! time {
    ($stat:expr, || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, vec![], || $block);
        }
    };
    ($stat:expr, move || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, vec![], || $block);
        }
    };
    ($stat:path, || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), vec![], || $block);
        }
    };
    ($stat:path, move || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), vec![], || $block);
        }
    };
    ($stat:expr, || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:expr, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:path, || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:path, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
}

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! time {
    ($stat:expr, || $block:expr) => {
        let _ = $stat;
        let _ = $block;
    };
    ($stat:expr, move || $block:expr) => {
        let _ = $stat;
        let _ = $block;
    };
    ($stat:path, || $block:expr) => {
        let _ = $stat;
        let _ = $block;
    };
    ($stat:path, move || $block:expr) => {
        let _ = $stat;
        let _ = $block;
    };
    ($stat:expr, || $block:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $block;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:expr, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $block;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, || $block:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $block;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $block;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];};
}
