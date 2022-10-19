# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/primait/prima_datadog.rs/compare/0.3.1...HEAD
[0.3.1]: https://github.com/primait/prima_datadog.rs/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/primait/prima_datadog.rs/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/primait/prima_datadog.rs/compare/0.1.9...0.2.0
