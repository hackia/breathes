# Breathes

**Breathes** is a Rust library designed to automate and facilitate source code analysis as well as user input validation (especially for commit messages). It allows for detecting the languages used in a project and executing verification "hooks" (tests, linting, security audits, etc.) in parallel.

## Features

### 1. Automated Verification Hooks
Breathes automatically detects your project's language and suggests appropriate hooks:
- **Rust**: `cargo check`, `cargo fmt`, `cargo clippy`, `cargo audit`, `cargo test`, `cargo doc`, `cargo outdated`.
- **Python**: `pip list --outdated`, `pip audit`.
- **Go**: `go test`, `go list` (security).
- **JavaScript/TypeScript**: `npm test`, `npm run lint`.
- **PHP**: `composer check-platform-reqs`, `composer audit`.
- **And many others**: Support for C#, Swift, Dart, Ruby, CMake, Elixir, Haskell, D, Kotlin, etc.

Hooks are executed in parallel using `rayon` for optimal performance, with an interactive progress bar provided by `indicatif`.

### 2. Input Validators
The library also provides a simple API for validating user input.
Breathes includes a `validator` module compatible with the `inquire` crate, allowing to validate:
- **Conventional Commits**: Verifies the commit type (`feat`, `fix`, `docs`, etc.).
- **Spelling**: Integration with Hunspell to check the spelling of messages.
- **Format**: Emails, passwords.
- **Style Rules**: Summary length (max 50 characters), punctuation, body line length (max 72 characters).

## Installation

Add Breathes to your `Cargo.toml`:

```toml
[dependencies]
breathes = "0.1.3"
```

*Note: Some features (like spell checking) require Hunspell dictionaries to be installed.*

## Usage

### Running Hooks

```rust
use breathes::hooks::run_hooks;

fn main() {
    match run_hooks() {
        Ok(exit_code) => println!("Hooks finished with code: {}", exit_code),
        Err(e) => eprintln!("Error while running hooks: {}", e),
    }
}
```

### Using Validators (with inquire)

```rust
use breathes::validator::validate_commit_type;
use inquire::Text;

fn main() {
    let commit_type = Text::new("Commit type?")
        .with_validator(validate_commit_type)
        .prompt();
}
```

## License

This project is licensed under **AGPL-3.0**.
