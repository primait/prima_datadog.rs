/// Make an arbitrary change to a StatsD counter
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! count {
    ($stat:expr, $count:expr) => {
        $crate::Datadog::count($stat, $count, vec![]);
    };
    ($stat:path, $count:expr) => {
        $crate::Datadog::count($stat.as_ref(), $count, vec![]);
    };
    ($stat:expr, $count:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::count($stat, $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $count:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::count($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
