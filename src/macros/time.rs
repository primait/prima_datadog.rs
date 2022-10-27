/// Time a block of code (reports in ms)
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! time {
    ($stat:expr, || $block:expr) => {
        $crate::Datadog::time($stat, &[], || $block);
    };
    ($stat:expr, move || $block:expr) => {
        $crate::Datadog::time($stat, &[], || $block);
    };
    ($stat:path, || $block:expr) => {
        $crate::Datadog::time($stat.as_ref(), &[], || $block);
    };
    ($stat:path, move || $block:expr) => {
        $crate::Datadog::time($stat.as_ref(), &[], || $block);
    };
    ($stat:expr, || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat, &[$(std::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
    ($stat:expr, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat, &[$(std::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
    ($stat:path, || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat.as_ref(), $count, &[$(std::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
    ($stat:path, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::time($stat.as_ref(), $count, &[$(std::format!("{}:{}", $key, $value).as_str()), *], || $block);
    };
}
