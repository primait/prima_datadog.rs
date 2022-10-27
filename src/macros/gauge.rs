/// Report an arbitrary value as a gauge
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! gauge {
    ($stat:expr, $val:expr) => {
        $crate::Datadog::gauge($stat, $val, &[]);
    };
    ($stat:path, $val:expr) => {
        $crate::Datadog::gauge($stat.as_ref(), $val, &[]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::gauge($stat, $val, &[$(std::concat!($key, ":", $value)), *]);
    };
    ($stat:path, $val:expr; $( $key:literal => $value:literal ), *) => {
        $crate::Datadog::gauge($stat.as_ref(), $val, &[$(std::concat!($key, ":", $value)), *]);
    };
    ($stat:expr, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::gauge($stat, $val, &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
    ($stat:path, $val:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::gauge($stat.as_ref(), $val, &[$(std::format!("{}:{}", $key, $value).as_str()), *]);
    };
}
