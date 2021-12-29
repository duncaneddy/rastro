# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

## [0.1.7] - 2021-12-28

### Fixed
- Fixed regression in path of `README.md` file in base rastro `Cargo.toml`

## [0.1.6] - 2021-12-28

### Added
- `anomaly_true_to_mean` and `anomaly_mean_to_true` anomaly conversion functions to enable 
  direct conversion between true and mean anomaly values.
- Added documentation example build and test pipeline to ensure all code examples in 
  documentation execute without error
- Added figure support to documentation. It is not possible to automatically generate and 
  include matplotlib and plotly figures as part of the documentation as part of the release process.

### Changed
- Overhauled documentation approach. Migrated to [material-mkdocs](https://squidfunk.github.io/mkdocs-material/)
for improved style and feature support.
- Moved from [*] style to (*) style units references in documentation to mitigate cargo doc warnings

### Fixed
- Made change to main `cargo.toml` file in attempt to get rust documentation publishing on docs.rs
- Fixed regression in release workflow

## [0.1.5] - 2021-03-21

### Changed
- Update release process to handle versioning and local development better.
  The `rastro_python` package is now kept in a state with a placeholder version
  of `0.0.0` and a relative path so that it can be developed against locally
  and it will work in the CI pipeline without having the rust package released.
  To release you now update the version in `rastro/Cargo.toml` and make a 
  tagged release on Github to trigger the release pipeline.

## [0.1.4] - 2021-03-21

### Added
- Added README to PyPi.org release
- Added badges to README

## [0.1.3] - 2021-03-21

### Fixed
- Attempt to fix issue with pip distribution of rust crate

## [0.1.2] - 2021-03-21

### Fixed
- Fixed issue in joint rust/python release where Python package versions weren't
  being properly updated.

## [0.1.1] - 2021-03-21

### Added
- Test new CI/CD pipleine

## [0.1.0] - 2021-12-28

### Added
- Initial release