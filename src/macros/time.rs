/// Time a block of code (reports in ms)
#[macro_export]
#[cfg(not(feature = "dev-null"))]
macro_rules! time {
    ($stat:literal, || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, vec![], || $block);
        }
    };
    ($stat:literal, move || $block:expr) => {
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
    ($stat:literal, || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:literal, move || $block:expr; $( $key:expr => $value:expr ), *) => {
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
#[cfg(feature = "dev-null")]
macro_rules! time {
    ($stat:literal, || $block:expr) => {
        let _ = $block;
    };
    ($stat:literal, move || $block:expr) => {
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
    ($stat:literal, || $block:expr; $( $key:expr => $value:expr ), *) => {
        let _ = $block;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:literal, move || $block:expr; $( $key:expr => $value:expr ), *) => {
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
