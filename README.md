# Obadh

Modern Bengali Input Method Engine

## Features

- High-performance Bengali text processing
- Cross-platform IME support
- Interactive debugging console
- Modular architecture

## Development

### Prerequisites

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building

```bash
# Build everything
cargo build

# Run tests
cargo test

# Run debug console
cargo run -p obadh-cli
```

## Related Projects

The following projects implement user interfaces using the Obadh engine:

- [obadh-qt](https://github.com/nsssayom/obadh-qt) - Qt configuration GUI
- [obadh-android](https://github.com/nsssayom/obadh-android) - Android keyboard
- [obadh-ios](https://github.com/nsssayom/obadh-ios) - iOS keyboard
- [obadh-web](https://github.com/nsssayom/obadh-web) - Web interface

## Contributing

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for development guidelines.

## License

MIT OR Apache-2.0
