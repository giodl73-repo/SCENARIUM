# Release readiness

SCENARIUM is package-ready at `0.1.0` for reproducible source consumption and
package inspection. Portfolio policy prohibits registering or publishing
crates on crates.io.

## Compatibility contract

- Minimum supported Rust version: 1.74.
- Rust API compatibility follows semantic versioning.
- Persisted documents use `scenarium.v1`.
- Compatible releases retain readable fixtures for every supported schema.
- Removing or reinterpreting persisted fields requires a new schema and a
  documented migration path.
- No optional features exist.
- Runtime dependencies are limited to `serde` and `serde_json`; both support
  the declared MSRV.

`Cargo.lock` remains at format version 3 so Cargo 1.74 can consume locked builds.

## Release gates

```powershell
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo doc --locked --no-deps
cargo package --locked
rustup run 1.74.0 cargo test --locked
```

The Rust 1.74 lane is evidence only when the exact toolchain is installed and
the command completes. If `rustup` is unavailable or the 1.74 toolchain is
missing, use the recorded CI evidence or install the exact toolchain before
claiming current MSRV validation.

The retained `scenarium.v1` and SIGNALS fixtures are included in the package.
CI runs both the stable toolchain gates and the Rust 1.74 test lane.

## Evidence boundary

Release readiness is supported by:

- invariant and adversarial contract tests;
- RALLY compatibility evidence;
- real SIGNALS sidecar interchange;
- direct CERES adoption with measured production-code deletion.

This evidence supports the current neutral contract. It does not permit
registry publication, automatic RALLY consumer migration, or new domain policy
inside SCENARIUM.
