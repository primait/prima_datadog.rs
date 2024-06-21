# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Changed

- *Breaking*: `with_country` will now add the `prima_country` tag instead of `prima:country`

---

## [0.7.2] - 2024-05-03

### Added

- Default tag set has new values based on the environment variables:
  * KUBE_APP_PART_OF
  * KUBE_APP_MANAGED_BY
  * KUBE_APP_VERSION
  * KUBE_APP_INSTANCE

---


## [0.7.1] - 2024-03-05

### Added

- Implemented `From<Configuration>` for `dogstatsd::Options`

### Changed

- `Config` is now `Configuration`

### Removed

- The `Configuration` trait
- `Country` and `Environment` serialization

---

## [0.7.0] - 2024-03-04

### Changed

- **BREAKING CHANGES**:
  - `PrimaConfiguration` is now `Config`
  - `Config::new` now takes only 2 arguments, `to_addr` and `namespace`
  - `from_addr` is now `0.0.0.0:0` by default, but can be customized using `.with_from_addr`

---

## [0.6.0] - 2023-10-02

### Added

- Socket path and batching options can now be optionally set as part of the configuration
- `async_time` macro to collect timing with asynchronous functions.
- `PrimaConfiguration` new function `with_environment` to optionally add an env.

### Removed

- **BREAKING CHANGE**: `PrimaConfiguration::new` function no longer takes `env` parameter.
  - Use `with_environment` function to manually set the `env:{env}` tag
  - If no environment is set the library will use `DD_ENV` var by default if set.

## [0.5.0] - 2022-12-05

### Added

- A `TimingGuard` guard struct, for more flexible execution time tracking
- The `compare!` macro, for running experiments in production

### Changed

- Fixed 0.4.0 backwards compatibility, relating to errors in some macro definitions
- Fixed various documentation errors

---

## [0.4.0] - 2022-11-21

### Added

- Added support for custom metric tracking via tag cardinality. See documentation in library header and `TagTrackerConfiguration`
- No longer allocate when it can be avoided

---

## [0.3.1] - 2022-10-19

### Added

- Custom tags are now supported through `PrimaConfiguration.with_tag`
- Country tag is now supported through `PrimaConfiguration.with_country`
- New warning in docs for high cardinality tags

---

## [0.3.0]

### Changed

- `Datadog::new` reintroduced `is_reporting_enabled`.
- `Datadog::new` is now a private function for tests.
- (Behaviour): The client, if not initialized, don't send metrics and don't panic.

### Removed

- `Datadog::global` function has been removed.

---

## [0.2.0]

### Changed

- `Datadog::new` removed `is_reporting_enabled` parameter

### Fixed

- Linting on `from_addr` function for `clippy::wrong_self_convention` check.

### Removed

- feature `noop`

---

## [0.1.9]

---

## [0.1.7]

---

## [0.1.6]

---

## [0.1.5]


[Unreleased]: https://github.com/primait/prima_datadog.rs/compare/0.7.2...HEAD
[0.7.2]: https://github.com/primait/prima_datadog.rs/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/primait/prima_datadog.rs/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/primait/prima_datadog.rs/compare/0.7.0...0.7.0
[0.6.0]: https://github.com/primait/prima_datadog.rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/primait/prima_datadog.rs/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/primait/prima_datadog.rs/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/primait/prima_datadog.rs/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/primait/prima_datadog.rs/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/primait/prima_datadog.rs/compare/0.1.9...0.2.0
[0.1.9]: https://github.com/primait/prima_datadog.rs/compare/0.1.7...0.1.9
[0.1.7]: https://github.com/primait/prima_datadog.rs/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/primait/prima_datadog.rs/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/primait/prima_datadog.rs/releases/tag/0.1.5
