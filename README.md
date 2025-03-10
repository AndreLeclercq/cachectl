# cachectl

A TUI (Terminal User Interface) application for managing cache directories on Linux systems.

## Description
`cachectl` allows you to:

- Browse ~/.cache directory
- View directory sizes
- Navigate through folders
- Delete selected files
- Monitor total disk usage

## Status
⚠️ This project is currently in early development and not suitable for production use. It serves as a learning project for Rust programming language. ⚠️

## Requirements

- Arch Linux
- Rust toolchain

## Building
```shell
cargo build
```

## Testing
```shell
cargo test
```

### Testing Safety Note
⚠️ **Important:** The test suite modifies environment variables using `std::env::set_var` and `std::env::remove_var`, which are marked as unsafe in multithreaded contexts. For safety, tests are configured to run in single-threaded mode via the `[package.metadata.cargo-test-options]` setting in `Cargo.toml`. Do not override this setting when running tests, as it could lead to undefined behavior.

## License
MIT 

## Contributing
As this is a learning project, issues and pull requests will be reviewed but might not be accepted to maintain the educational focus.
