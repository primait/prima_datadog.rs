/// Report a value in a set
#[macro_export]
macro_rules! service_check {
    // call with literal and status
    ($stat:literal, $service_status:path) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, vec![], None);
        }
    };
    ($stat:literal, $service_status:path, $options: ident) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, vec![], Some($options));
        }
    };
    ($stat:literal, $service_status:path, $options: expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, vec![], Some($options));
        }
    };
    // call with path, status and options
    ($stat:path, $service_status:path) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, vec![], None);
        }
    };
    ($stat:path, $service_status:path, $options: ident) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, vec![], Some($options));
        }
    };
    ($stat:path, $service_status:path, $options: expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, vec![], Some($options));
        }
    };
    // call with literal, status, options and tags
    ($stat:literal, $service_status:path; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], None);
        }
    };
    ($stat:literal, $service_status:path, $options: ident; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
        }
    };
    ($stat:literal, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
        }
    };
    // call with path, status, options and tags
    ($stat:path, $service_status:path; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], None);
        }
    };
    ($stat:path, $service_status:path, $options: ident; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
        }
    };
    ($stat:path, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
        }
    };
}
