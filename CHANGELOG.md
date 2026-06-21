# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-06-21

### Added
- Initial release of hace-me-race-cli
- `RaceCliDispatcher` trait for Bridge pattern implementation
- `RaceEventObserver` trait for Observer pattern implementation
- `CliObserver` struct implementing `RaceEventObserver`
- `ExecutionConfig` for dynamic territory/endpoint injection (Era 5 compliant)
- Endpoint validation with colon check (`:`)
- `register_in_caw` function for CAW artifact registration
- `CHANGELOG.md` for version tracking

### Security
- Input validation on endpoint format
- Path sanitization for artifact operations