/// Report the status of a service
#[macro_export]
#[cfg(not(feature = "noop"))]
macro_rules! service_check {
    // call with literal and status
    ($stat:expr, $service_status:path) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, vec![], None);
        }
    };
    ($stat:expr, $service_status:path, $options: expr) => {
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
    ($stat:path, $service_status:path, $options: expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, vec![], Some($options));
        }
    };
    // call with literal, status, options and tags
    ($stat:expr, $service_status:path; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat, $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], None);
        }
    };
    ($stat:expr, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
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
    ($stat:path, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().service_check($stat.as_ref(), $service_status, std::vec![$(std::format!("{}:{}", $key, $value)), *], Some($options));
        }
    };
}

#[macro_export]
#[cfg(feature = "noop")]
macro_rules! service_check {
    ($stat:expr, $service_status:path) => {
        let _ = $stat;
        let _ = $service_status;
    };
    ($stat:expr, $service_status:path, $options: ident) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
    };
    ($stat:expr, $service_status:path, $options: expr) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
    };
    ($stat:path, $service_status:path) => {
        let _ = $stat;
        let _ = $service_status;
    };
    ($stat:path, $service_status:path, $options: ident) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
    };
    ($stat:path, $service_status:path, $options: expr) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
    };
    ($stat:expr, $service_status:path; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:expr, $service_status:path, $options: ident; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:expr, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $service_status:path; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $service_status:path, $options: ident; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
    ($stat:path, $service_status:path, $options: expr; $( $key:expr => $value:expr ), *) => {
        let _ = $stat;
        let _ = $service_status;
        let _ = $options;
        let _ = std::vec![$(std::format!("{}:{}", $key, $value)), *];
    };
}
