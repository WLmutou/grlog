# grlog

A high-performance async logging library based on gorust GMP runtime.

## Overview

`grlog` is a high-performance asynchronous logging library built on top of the `gorust` GMP runtime. It leverages Go-like concurrency models to provide efficient, non-blocking logging capabilities for Rust applications.

The library is designed to handle high-volume logging scenarios where traditional synchronous loggers might become bottlenecks. By utilizing gorust's lightweight goroutines and channels, `grlog` ensures logging doesn't block your main application threads.

## Features

- **Asynchronous**: Non-blocking logging that won't stall your application
- **High Performance**: Built on gorust GMP runtime for efficient concurrency
- **Flexible Configuration**: Support for multiple log targets and filtering
- **Environment Integration**: Compatible with standard RUST_LOG environment variable
- **Module-level Filtering**: Configure log levels per module
- **Multiple Output Targets**: Console (stdout/stderr), file, and other outputs supported
- **Timestamp Formatting**: Automatic timestamp with millisecond precision
- **Self-contained**: No external logging dependencies required

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grlog = "0.1.0"
```

## Usage

### Basic Usage

```rust
use grlog::{init, info, debug, warn, error};

fn main() {
    // Initialize with default settings (logs to stderr, level info)
    init();
    
    info!("This is an info message!");
    warn!("This is a warning!");
    error!("This is an error!");
}
```

### Advanced Configuration

```rust
use grlog::{builder, Target, LevelFilter};

fn main() {
    // Create a custom logger with specific settings
    let mut builder = builder();
    builder
        .filter_level(LevelFilter::Debug)
        .filter_module("hyper", LevelFilter::Info)
        .filter_module("my_crate::network", LevelFilter::Trace)
        .target(Target::Stdout)
        .buffer_size(2048);
    
    builder.init().unwrap();
    
    info!("This is an info message!");
    debug!("This is a debug message");
    warn!("This is a warning");
}
```

### Logging to a File

```rust
use grlog::{builder, Target, info};
use std::path::PathBuf;

fn main() {
    let mut log_path = PathBuf::from("/tmp");
    log_path.push("app.log");
    
    let mut builder = builder();
    builder
        .target(Target::File(log_path))
        .filter_level(LevelFilter::Info);
    
    builder.init().unwrap();
    
    info!("This will be written to the file!");
}
```

### Using Environment Variables

```rust
use grlog::init_from_env;

fn main() {
    // Initialize from RUST_LOG environment variable
    // Example: RUST_LOG=debug,cargo=info,my_module=trace cargo run
    init_from_env();
    
    info!("This is an info message!");
}
```

## Configuration Options

- **Targets**: Choose between stdout, stderr, or file output
- **Log Levels**: Trace, Debug, Info, Warn, Error, or Off
- **Module Filters**: Set different log levels for different modules
- **Buffer Size**: Configure the size of the async channel buffer

### Log Format

All log messages follow this format:
```
YYYY-MM-DD HH:MM:SS.mmm LEVEL [target] message
```

Example:
```
2023-05-14 10:30:45.123 INFO [my_app] Application started successfully
```

## Architecture

The core of `grlog` is built around:

1. **Gorust GMP Runtime**: Provides Go-like concurrency with goroutines
2. **Async Backend**: Uses channels for non-blocking message passing
3. **Log Writer Interface**: Pluggable writers for different output targets
4. **Formatter**: Standardized log message formatting with timestamps
5. **Builder Pattern**: Flexible configuration API similar to env_logger

The architecture ensures that even if the logging backend is temporarily slow, your application continues to run without blocking.

## Performance

By leveraging gorust's GMP (Goroutine, Monitor, Processor) model, `grlog` provides:

- Extremely lightweight goroutines for log processing
- Non-blocking message passing through channels
- Minimal overhead during log emission
- Efficient batching and buffering mechanisms
- Thread-safe concurrent logging from multiple application threads

## Comparison with Other Loggers

Unlike traditional Rust loggers that typically write synchronously to output streams, `grlog` uses an asynchronous approach that separates the act of requesting a log from the actual writing. This prevents I/O bottlenecks from affecting your application's performance.

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For bug reports or feature requests, open an issue on GitHub.

## Changelog

### v0.1.0
- Initial release
- Asynchronous logging using gorust GMP runtime
- Support for stdout, stderr and file output
- Module-specific log level filtering
- Environment variable configuration support
- Timestamped log entries with millisecond precision
