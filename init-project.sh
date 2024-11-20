#!/bin/bash

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m'

print_status() { echo -e "${BLUE}${BOLD}==> ${NC}$1"; }
print_success() { echo -e "${GREEN}✓ ${NC}$1"; }
print_warning() { echo -e "${YELLOW}! ${NC}$1"; }
print_error() { echo -e "${RED}✗ ${NC}$1"; }

show_help() {
    cat << EOF
Usage: ./init-project.sh [OPTIONS]

Initialize the Obadh project structure.

Options:
    -f, --force     Force initialization (warning: deletes existing structure)
    -h, --help      Show this help message

Creates a complete project structure for the Obadh Bengali Input Method Engine.
EOF
    exit 0
}

check_project_exists() {
    [ -d "crates" ] || [ -f "Cargo.toml" ]
}

# Create module structure for a library crate
create_lib_structure() {
    local CRATE_PATH=$1
    mkdir -p "$CRATE_PATH/src"/{error,types,utils}
    
    # Create basic module files with proper documentation
    cat > "$CRATE_PATH/src/lib.rs" << EOL
//! ${CRATE_PATH##*/} - Part of the Obadh Bengali Input Method
//!
//! This module provides core functionality for the input method engine.

pub mod error;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert!(true);
    }
}
EOL

    cat > "$CRATE_PATH/src/error/mod.rs" << EOL
//! Error types and handling

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("System error: {0}")]
    SystemError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
EOL

    touch "$CRATE_PATH/src/types/mod.rs"
    touch "$CRATE_PATH/src/utils/mod.rs"
}

# Enhanced crate creation with proper dependencies
create_crate() {
    local CRATE_PATH=$1
    local CRATE_NAME=$2
    local CRATE_TYPE=$3  # lib or bin
    
    mkdir -p "$CRATE_PATH"
    
    # Create Cargo.toml with appropriate dependencies
    cat > "$CRATE_PATH/Cargo.toml" << EOL
[package]
name = "$CRATE_NAME"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Part of the Obadh Bengali Input Method Engine"

[dependencies]
log.workspace = true
thiserror.workspace = true
serde = { workspace = true, features = ["derive"] }

[dev-dependencies]
test-case.workspace = true
pretty_assertions.workspace = true
criterion.workspace = true
EOL

    if [ "$CRATE_TYPE" = "lib" ]; then
        create_lib_structure "$CRATE_PATH"
    else
        mkdir -p "$CRATE_PATH/src"/{commands,debug,repl}
        # Create main.rs for CLI
        cat > "$CRATE_PATH/src/main.rs" << EOL
//! Obadh Debug Console
//! 
//! Interactive debugging and testing tool

mod commands;
mod debug;
mod repl;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Obadh Bengali IME - Debug Console v{}", env!("CARGO_PKG_VERSION"));
    
    let mut input = String::new();
    loop {
        print!("obadh> ");
        io::stdout().flush()?;
        
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        match input {
            "exit" | "quit" => break,
            "help" => show_help(),
            _ => process_input(input),
        }
    }
    
    Ok(())
}

fn show_help() {
    println!("Commands:");
    println!("  help    Show this help");
    println!("  exit    Exit the console");
    println!("  quit    Exit the console");
}

fn process_input(input: &str) {
    println!("Processing: {}", input);
}
EOL
    fi
}

main() {
    local FORCE=false

    case "$1" in
        -h|--help) show_help ;;
        -f|--force) FORCE=true ;;
        "") ;;
        *) print_error "Unknown option: $1"; show_help ;;
    esac

    if check_project_exists; then
        if [ "$FORCE" = true ]; then
            print_warning "Existing project structure detected. Forcing reinitialization..."
            rm -rf crates docs tests examples .github .vscode .cargo *.toml .gitignore
        else
            print_error "Project structure already exists. Use --force to reinitialize."
            exit 1
        fi
    fi

    # Create main project structure
    print_status "Creating project structure..."
    
    # Core functionality
    mkdir -p crates/core/{bengali/{src/{composition,dictionary,phonetic},tests},engine/{src/{cache,processor},tests},utils/src}
    
    # Platform support
    mkdir -p crates/platforms/{linux/{ibus,fcitx,wayland},windows/tsf,macos/input_method}
    
    # Protocols
    mkdir -p crates/protocols/{ime/{src/{traits,events},tests},config/src/schema}
    
    # Testing and examples
    mkdir -p {tests/{integration,benchmark},examples/{basic,platforms}}
    
    # Tools
    mkdir -p crates/tools/cli

    # Configuration and documentation
    mkdir -p {.cargo,.github/workflows,docs/{api,design}}

    print_status "Initializing crates..."
    
    # Initialize core crates
    create_crate "crates/core/bengali" "obadh-bengali" "lib"
    create_crate "crates/core/engine" "obadh-engine" "lib"
    create_crate "crates/core/utils" "obadh-utils" "lib"
    
    # Initialize protocol crates
    create_crate "crates/protocols/ime" "obadh-ime" "lib"
    create_crate "crates/protocols/config" "obadh-config" "lib"
    
    # Initialize platform crates
    create_crate "crates/platforms/linux/ibus" "obadh-linux-ibus" "lib"
    create_crate "crates/platforms/linux/fcitx" "obadh-linux-fcitx" "lib"
    create_crate "crates/platforms/linux/wayland" "obadh-linux-wayland" "lib"
    create_crate "crates/platforms/windows/tsf" "obadh-windows-tsf" "lib"
    create_crate "crates/platforms/macos/input_method" "obadh-macos-ime" "lib"
    
    # Initialize CLI
    create_crate "crates/tools/cli" "obadh-cli" "bin"

    # Create root Cargo.toml
    print_status "Creating workspace configuration..."
    cat > Cargo.toml << EOL
[workspace]
resolver = "2"
members = [
    "crates/core/bengali",
    "crates/core/engine",
    "crates/core/utils",
    "crates/protocols/ime",
    "crates/protocols/config",
    "crates/platforms/linux/ibus",
    "crates/platforms/linux/fcitx",
    "crates/platforms/linux/wayland",
    "crates/platforms/windows/tsf",
    "crates/platforms/macos/input_method",
    "crates/tools/cli"
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Nazmus Shakib Sayom <sayom.shakib@utah.edu>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/nsssayom/obadh.git"

[workspace.dependencies]
# Logging & Error Handling
log = "0.4"
env_logger = "0.10"
thiserror = "1.0"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Testing & Benchmarking
test-case = "3.1"
pretty_assertions = "1.4"
criterion = "0.5"

# Unicode & Text Processing
unicode-segmentation = "1.10"

# Terminal UI
crossterm = "0.27"
EOL

    # Create cargo config
    cat > .cargo/config.toml << EOL
[build]
rustflags = ["-D", "warnings"]

[target.'cfg(all())']
rustflags = ["-C", "target-cpu=native"]
EOL

    # Create rustfmt configuration
    cat > rustfmt.toml << EOL
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
EOL

    # Create clippy configuration
    cat > clippy.toml << EOL
cognitive-complexity-threshold = 20
too-many-arguments-threshold = 10
too-many-lines-threshold = 150
EOL

    # Create GitHub workflow
    mkdir -p .github/workflows
    cat > .github/workflows/ci.yml << EOL
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: \${{ matrix.os }}
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        components: rustfmt, clippy
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
    - name: Run clippy
      run: cargo clippy -- -D warnings
    - name: Check formatting
      run: cargo fmt -- --check
EOL

    # Create gitignore
    cat > .gitignore << EOL
/target
**/*.rs.bk
Cargo.lock
.env
*.log
.DS_Store
.idea
.vscode/*
!.vscode/settings.json
EOL

    # Create main README
    cat > README.md << EOL
# Obadh

Modern Bengali Input Method Engine

## Features

- High-performance Bengali text processing
- Cross-platform IME support
- Interactive debugging console
- Modular architecture

## Development

### Prerequisites

\`\`\`bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
\`\`\`

### Building

\`\`\`bash
# Build everything
cargo build

# Run tests
cargo test

# Run debug console
cargo run -p obadh-cli
\`\`\`

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
EOL

    # Initialize git
    if [ ! -d ".git" ]; then
        print_status "Initializing git repository..."
        git init
        git add .
        git commit -m "Initial project setup"
    fi

    print_success "Project initialized successfully!"
    echo
    print_status "Next steps:"
    echo "1. cargo build                  # Build all crates"
    echo "2. cargo run -p obadh-cli       # Run debug console"
    echo "3. git remote add origin https://github.com/nsssayom/obadh.git"
}

main "$@"