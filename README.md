# grlog

A high-performance async logging library based on gorust GMP runtime.

## Overview

`grlog` is a high-performance asynchronous logging library built on top of the `gorust` GMP (Goroutine-Monitor-Processor) runtime. It leverages Go-like concurrency models to provide efficient, non-blocking logging capabilities for Rust applications.

The library is designed to handle high-volume logging scenarios where traditional synchronous loggers might become bottlenecks. By utilizing gorust's lightweight goroutines and channels, `grlog` ensures logging doesn't block your main application threads.

## Features

- **Asynchronous**: Non-blocking logging via gorust channel + GMP goroutine
- **High Performance**: Built on gorust GMP runtime for efficient concurrency
- **Flexible Configuration**: Fluent builder API with multiple log targets and filtering
- **Environment Integration**: Compatible with standard `RUST_LOG` environment variable
- **Module-level Filtering**: Configure log levels per module
- **Multiple Output Targets**: Console (stdout/stderr), file output supported
- **Timestamp Formatting**: Automatic timestamp with millisecond precision
- **Custom Log Trait**: Implements its own `Log` trait for maximum flexibility

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grlog = "0.1.1"
```

## Usage

### Basic Usage

```rust
use grlog::{init, info, warn, error};

fn main() {
    // Initialize with default settings (logs to stderr, level info)
    init();

    info!("This is an info message!");
    warn!("This is a warning!");
    error!("This is an error!");
}
```

### Available Log Macros

```rust
use grlog::{trace, debug, info, warn, error};

fn main() {
    grlog::init();

    trace!("Trace level - finest granularity");
    debug!("Debug level - diagnostic information");
    info!("Info level - general information");
    warn!("Warn level - potentially harmful situations");
    error!("Error level - error events");
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

    // Option 1: init with error handling
    builder.init().unwrap();

    // Option 2: try_init to silently ignore if already initialized
    // builder.try_init();

    info!("This is an info message!");
    debug!("This is a debug message");
}
```

### Logging to a File

```rust
use grlog::{builder, Target, info};
use std::path::PathBuf;

fn main() {
    let log_path = PathBuf::from("/tmp/app.log");

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

### Custom Environment Variable Name

```rust
use grlog::builder;

fn main() {
    // Use a custom environment variable name instead of RUST_LOG
    let mut builder = builder().from_env("MY_APP_LOG");
    builder.init().unwrap();

    info!("Initialized from MY_APP_LOG env var!");
}
```

### Flushing Logs

The `Log` trait provides a `flush()` method that ensures all pending log messages are written before exit. The logger's `flush()` method triggers the async backend to drain remaining messages in the channel and flush the underlying writer.

## Configuration Options

- **Targets**: Choose between `Target::Stdout`, `Target::Stderr` (default), or `Target::File(PathBuf)`
- **Log Levels**: `Trace`, `Debug`, `Info` (default), `Warn`, `Error`, `Off`
- **Module Filters**: Set different log levels for specific modules using `filter_module()`
- **Buffer Size**: Configure the size of the async channel buffer (default: 1024)

### Log Format

All log messages follow this format:

```
YYYY-MM-DD HH:MM:SS.mmm LEVEL [target] message
```

Example:

```
2023-05-14 10:30:45.123 INFO  [my_app] Application started successfully
2023-05-14 10:30:45.456 DEBUG [my_app::network] Connected to server
2023-05-14 10:30:45.789 WARN  [my_app::db] Connection pool at 80% capacity
```

## Architecture

The core of `grlog` is built around these components:

### Components

1. **Gorust GMP Runtime** — Provides Go-like concurrency with lightweight goroutines for background log processing
2. **Async Backend** (`AsyncLogBackend`) — Uses gorust channels for non-blocking message passing between application threads and the log writer goroutine
3. **LogWriter Trait** — Pluggable interface for different output targets (`StdoutWriter`, `StderrWriter`, `FileWriter`)
4. **Formatter** — Standardized log message formatting with timestamps (millisecond precision via `chrono`)
5. **Builder Pattern** — `LoggerBuilder` provides a flexible configuration API similar to `env_logger`
6. **Logger** (`GrLogger`) — Implements the `Log` trait with module-level filtering support

### Data Flow

```
Application Thread
    │
    ▼
_log() → checks MAX_LEVEL → checks GLOBAL_LOGGER.enabled()
    │
    ▼
GrLogger.log() → filter by module/global level → format message
    │
    ▼
AsyncLogBackend.send() → channel.try_send()
    │
    ▼
Goroutine (background) → channel.try_recv() → LogWriter.write()
```

The architecture ensures that even if the logging backend is temporarily slow, your application continues to run without blocking. The background goroutine drains the channel and writes to the configured output target.

## API Reference

### Core Types

| Type | Description |
|------|-------------|
| `Level` | Log level enum: `Trace`, `Debug`, `Info`, `Warn`, `Error` |
| `LevelFilter` | Level filter with an additional `Off` variant |
| `Target` | Output target: `Stdout`, `Stderr` (default), `File(PathBuf)` |
| `Metadata` | Log metadata containing level and target module path |
| `Record` | Complete log record with metadata and formatted arguments |
| `Log` | Trait for implementing custom loggers (`enabled`, `log`, `flush`) |
| `LoggerBuilder` | Fluent builder for configuring and initializing the logger |
| `SetLoggerError` | Error returned when logger is already initialized |
| `LogWriter` | Trait for implementing custom output writers |

### Public Functions

| Function | Description |
|----------|-------------|
| `init()` | Initialize with default config (stderr, level info) |
| `init_from_env()` | Initialize from `RUST_LOG` environment variable |
| `builder()` | Create a new `LoggerBuilder` |

### Builder Methods

| Method | Description |
|--------|-------------|
| `filter_level(LevelFilter)` | Set the global log level filter |
| `filter_module(&str, LevelFilter)` | Set module-specific log level |
| `target(Target)` | Set the output target |
| `buffer_size(usize)` | Set async channel buffer capacity |
| `from_env(&str)` | Load configuration from a custom env var |
| `init()` | Initialize the logger (returns `Result`) |
| `try_init()` | Initialize the logger (silently ignores errors) |

### Log Macros

| Macro | Description |
|-------|-------------|
| `trace!()` | Log at TRACE level |
| `debug!()` | Log at DEBUG level |
| `info!()` | Log at INFO level |
| `warn!()` | Log at WARN level |
| `error!()` | Log at ERROR level |

## Performance

By leveraging gorust's GMP (Goroutine-Monitor-Processor) model, `grlog` provides:

- Extremely lightweight goroutines for log processing
- Non-blocking message passing through channels
- Minimal overhead during log emission
- Thread-safe concurrent logging from multiple application threads

## Comparison with Other Loggers

Unlike traditional Rust loggers that typically write synchronously to output streams, `grlog` uses an asynchronous approach that separates the act of requesting a log from the actual writing. This prevents I/O bottlenecks from affecting your application's performance.

## Dependencies

- **[gorust](https://github.com/WLmutou/gorust)** — GMP runtime providing goroutines and channels
- **[chrono](https://crates.io/crates/chrono)** — Timestamp formatting with millisecond precision

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For bug reports or feature requests, open an issue on GitHub.