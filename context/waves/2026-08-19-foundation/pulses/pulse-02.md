# Pulse 02: Contract hardening

## Goal

Complete VTRACE work package `WP-001` so consumer compatibility work targets a
versioned and adversarially tested contract.

## Product changes

- Split the single source file into focused internal modules without splitting
  the published crate.
- Add explicit schema versioning and retained golden fixtures.
- Validate packet references and optional artifact digests.
- Complete improved, regressed, mixed, equivalent, invalid, and incomplete
  comparison cases.
- Document compatibility policy.

## Non-goals

- No RALLY migration.
- No SIGNALS methodology or runtime dependency.
- No additional published crates.
- No crates.io publication.

## Validation

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo test --doc`
- `cargo package --allow-dirty`
- VTRACE validation

## Status

Complete.
