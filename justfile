#!/usr/bin/env -S just --justfile
# copied from https://github.com/oxc-project/oxc/blob/1f8968a52105fef6d2f703c5e0d6589e415f9866/justfile

alias r := ready

# When ready, run the same CI commands
ready:
  git diff --exit-code --quiet
  typos
  just fmt
  just test
  just lint
  cargo check
  git status

# Format all files
fmt:
  cargo fmt
  taplo format

# Run all the tests
test:
  cargo test

# Lint the whole project
lint:
  cargo clippy -- --deny warnings
