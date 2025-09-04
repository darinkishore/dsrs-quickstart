# Claude Instructions for DSRs Quickstart

This document contains specific instructions for Claude when working with this repository.

## Project Overview
This is a quickstart template for DSRs (dspy-rs), a Rust implementation of the DSPy framework for building LLM pipelines. The DSRs library is vendored in the `vendor/` directory for immediate use without external dependencies.

## Key Guidelines

### 1. Dependency Management
- The DSRs library is **vendored** in `vendor/` - do not try to use cargo.io versions
- When updating DSRs, always:
  1. Clone from https://github.com/krypticmouse/DSRs
  2. Remove the `.git` directory from vendor
  3. Fix Rust edition issues (2024 → 2021)
  4. Fix let-chain syntax for Rust 2021 compatibility
  5. Update `vendor/.git_sha` with the latest commit SHA

### 2. Testing
- Always run `cargo build` before committing changes
- Run `cargo test` to ensure integration tests pass
- Tests are located in `tests/integration_test.rs`
- When API key is not available, tests should gracefully handle it

### 3. Code Style
- Use Rust 2021 edition features (not 2024)
- Follow existing code patterns in `src/main.rs`
- Keep examples simple and focused on DSRs concepts

### 4. Common Tasks

#### Updating Vendored DSRs
```bash
./scripts/update-vendor.sh
```

#### Running Tests
```bash
cargo test
```

#### Building the Project
```bash
cargo build --release
```

### 5. DSRs-Specific Knowledge
- **Signatures**: Define LLM task contracts with `#[Signature]`
- **Predictors**: Use `Predict::new()` to create executors
- **Modules**: Implement the `Module` trait for complex pipelines
- **Examples**: Use the `example!` macro for structured inputs
- Always import `Module` trait when using `.forward()` method

### 6. Common Issues & Solutions

| Issue | Solution |
|-------|----------|
| `Module` trait not in scope | Add `use dspy_rs::Module;` |
| Edition compatibility errors | Ensure all Cargo.toml files use `edition = "2021"` |
| Let-chain syntax errors | Rewrite to nested if statements |
| Missing API key | Check `.env` file for `OPENAI_API_KEY` |

### 7. Pull Request Guidelines
When creating PRs for dependency updates:
- Title: "Update DSRs to [SHA]"
- Include before/after SHA in description
- Ensure all tests pass
- Verify example still runs correctly

### 8. GitHub Actions
The repository has automated workflows:
- `claude.yml`: Responds to @claude mentions in issues/PRs
- `update-dsrs.yml`: Daily checks for DSRs updates
- Both workflows require `ANTHROPIC_API_KEY` or `CLAUDE_CODE_OAUTH_TOKEN` secrets

## Important Commands

```bash
# Clone and setup
git clone https://github.com/darinkishore/dsrs-quickstart.git
cd dsrs-quickstart
cp .env.example .env  # Add your API key

# Development
cargo run              # Run the example
cargo test            # Run tests
cargo build --release # Build optimized

# Maintenance
./scripts/update-vendor.sh  # Update vendored DSRs
```

## Contact
Repository: https://github.com/darinkishore/dsrs-quickstart
DSRs Source: https://github.com/krypticmouse/DSRs