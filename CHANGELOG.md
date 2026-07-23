# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Updated README documentation with comprehensive API reference, architecture details, and accurate code examples

## [0.1.0] - 2026-05-14

### Added
- Initial release of grlog
- Asynchronous logging implementation using gorust GMP runtime
- Support for multiple output targets (stdout, stderr, file)
- Module-specific log level filtering
- Environment variable configuration support via RUST_LOG
- Standardized log formatting with timestamps (with millisecond precision)
- Builder pattern API for flexible logger configuration
- Implementation of the `log` crate traits
- Comprehensive test coverage for all major components

### Features
- Non-blocking logging that won't stall your application
- High-performance concurrency model using gorust GMP runtime
- Configurable buffer sizes for async channel
- Support for standard log levels (Trace, Debug, Info, Warn, Error)
- Default target is stderr, configurable to stdout or file
- Automatic directory creation for file targets