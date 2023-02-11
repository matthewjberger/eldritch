set windows-shell := ["powershell.exe"]

check:
  cargo check --all --tests
  cargo fmt --all --check

format:
  cargo fmt --all

lint:
  cargo clippy --all --tests

test:
  cargo test --workspace

@versions:
    rustc --version
    cargo fmt -- --version
    cargo clippy -- --version
