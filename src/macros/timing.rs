/// Send your own timing metric in milliseconds
/// NOTE: Try to minimise variation in tag values (avoid things like timestamps or ids). See note in lib docs!
#[macro_export]
macro_rules! timing {
    ($stat:expr, $ms:expr) => {
        $crate::Datadog::timing($stat, $ms, vec![]);
    };
    ($stat:path, $ms:expr) => {
        $crate::Datadog::timing($stat.as_ref(), $ms, vec![]);
    };
    ($stat:expr, $ms:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::timing($stat, $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
    ($stat:path, $ms:expr; $( $key:expr => $value:expr ), *) => {
        $crate::Datadog::timing($stat.as_ref(), $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
    };
}
