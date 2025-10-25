# Sleep Interactive

Like sleep but with a visual progress indicator and more explicit param names.

## Usage Example

    cargo r --release -- --minutes 30 && osascript -e 'display notification "Check your MRs build status" with title "MR" sound name "Submarine"'

## Install

    cargo install sleep-interactive

## Release/Publish to crates.io

    cargo bump --git-tag && git add . && git commit --amend && git push --tags

## All checks

    cargo build && cargo test && cargo bench && cargo fmt --check && cargo doc && cargo clippy && cargo clippy --all -- -W clippy::all -W clippy::correctness -W clippy::suspicious -W clippy::style -W clippy::complexity -W clippy::perf -W clippy::pedantic -W clippy::nursery -W clippy::cargo
