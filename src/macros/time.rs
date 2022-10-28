/// Time a block of code (reports in ms)
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! time {
    ($stat:expr, || $block:expr) => {
        $crate::Datadog::time($stat, $crate::EMPTY_TAGS, || $block);
    };
    ($stat:expr, move || $block:expr) => {
        $crate::Datadog::time($stat, $crate::EMPTY_TAGS, || $block);
    };
    ($stat:path, || $block:expr) => {
        $crate::Datadog::time($stat.as_ref(), $crate::EMPTY_TAGS, || $block);
    };
    ($stat:path, move || $block:expr) => {
        $crate::Datadog::time($stat.as_ref(), $crate::EMPTY_TAGS, || $block);
    };
    ($stat:expr, || $block:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::time($stat, &[$(::core::concat!($key, ":", $value)), *], || $block);
    };
    ($stat:expr, move || $block:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::time($stat, &[$(::core::concat!($key, ":", $value)), *], || $block);
    };
    ($stat:path, || $block:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::time($stat.as_ref(), $count, &[$(::core::concat!($key, ":", $value)), *], || $block);
    };
    ($stat:path, move || $block:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::time($stat.as_ref(), $count, &[$(::core::concat!($key, ":", $value)), *], || $block);
    };
    ($stat:expr, || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
    ($stat:expr, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
    ($stat:path, || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat.as_ref(), $count, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
    ($stat:path, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat.as_ref(), $count, &[$(::core::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
}
