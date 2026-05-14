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
- **Multiple Output Targets**: Console, file, and other outputs supported

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grlog = "0.1.0"
log = "0.4"
```

## Usage

### Basic Usage

```rust
use log::info;
use grlog::{init, builder};

fn main() {
    // Initialize with default settings
    init();
    
    info!("This is an info message!");
}
```

### Advanced Configuration

```rust
use log::{info, debug, warn};
use grlog::{builder, Target};

fn main() {
    // Create a custom logger with specific settings
    let mut builder = builder();
    builder
        .filter_level(log::LevelFilter::Debug)
        .filter_module("hyper", log::LevelFilter::Info)
        .target(Target::Stdout)
        .buffer_size(2048);
    
    builder.init().unwrap();
    
    info!("This is an info message!");
    debug!("This is a debug message");
    warn!("This is a warning");
}
```

### Using Environment Variables

```rust
use grlog::init_from_env;

fn main() {
    // Initialize from RUST_LOG environment variable
    // Example: RUST_LOG=debug,my_module=trace cargo run
    init_from_env();
}
```

## Configuration Options

- **Targets**: Choose between stdout, stderr, or file output
- **Log Levels**: Trace, Debug, Info, Warn, Error, or Off
- **Module Filters**: Set different log levels for different modules
- **Buffer Size**: Configure the size of the async channel buffer

## Architecture

The core of `grlog` is built around:

1. **Gorust GMP Runtime**: Provides Go-like concurrency with goroutines
2. **Async Backend**: Uses channels for non-blocking message passing
3. **Log Writer Interface**: Pluggable writers for different output targets
4. **Formatter**: Customizable log message formatting

The architecture ensures that even if the logging backend is temporarily slow, your application continues to run without blocking.

## Performance

By leveraging gorust's GMP (Goroutine, Monitor, Processor) model, `grlog` provides:

- Extremely lightweight goroutines for log processing
- Non-blocking message passing through channels
- Minimal overhead during log emission
- Efficient batching and buffering mechanisms

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

We welcome contributions! Please feel free to submit a Pull Request. For bug reports or feature requests, open an issue on GitHub.