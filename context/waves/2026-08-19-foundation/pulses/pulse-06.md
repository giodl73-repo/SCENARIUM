# Pulse 06: PITFALL Use-Case Pass

## Goal

Make SCENARIUM's open PITFALL entries usable by future maintainer, adopter,
release, and publication decisions rather than only recording process risk.

## Change

- Added actor, task, surface, likely mistake, consequence, owner, and test
  fields to `SCEN-PF-01` through `SCEN-PF-04`.
- Added retained policy coverage in `tests/pitfall_policy.rs`.
- Kept `SCEN-PF-01` through `SCEN-PF-04` open because the test preserves
  boundaries; it does not approve new consumer migration, registry publication,
  product policy, or external evidence truth claims.
- Preserved the release-readiness MSRV note that exact Rust 1.74 evidence
  requires the installed toolchain or recorded CI evidence.
- Added `docs/adoption-gates.v1.json` so compatibility, migration approval,
  registry publication, and SCENARIUM expansion state can be checked as data.

## Validation

```powershell
C:\Users\giodl\.cargo\bin\cargo.exe fmt --check
C:\Users\giodl\.cargo\bin\cargo.exe test --test pitfall_policy
C:\Users\giodl\.cargo\bin\cargo.exe test --locked
C:\Users\giodl\.cargo\bin\cargo.exe clippy --locked --all-targets -- -D warnings
C:\Users\giodl\.cargo\bin\cargo.exe doc --locked --no-deps
C:\Users\giodl\.cargo\bin\cargo.exe package --locked --allow-dirty
C:\Users\giodl\.cargo\bin\cargo.exe run --manifest-path C:\src\TRACKER\repos\standards-protocols\pitfall\Cargo.toml -q -p pitfall-cli -- validate C:\src\TRACKER\repos\tools-infra\scenarium --format json
python C:\src\TRACKER\repos\standards-protocols\pitfall\tools\check_pitfall.py C:\src\TRACKER\repos\tools-infra\scenarium
git diff --check -- .pitfall\scenarium-pitfalls.md docs\release-readiness.md tests\pitfall_policy.rs
```

`rustup run 1.74.0 cargo test --locked` was attempted and failed because
`1.74.0-x86_64-pc-windows-msvc` is not installed locally. Per
`docs/release-readiness.md`, that lane cannot be claimed as current local MSRV
evidence without installing the exact toolchain or citing recorded CI evidence.

## Result

SCENARIUM now has retained tests citing all four repo-local PITFALL risks:
neutral contract absorption, comparison-status recommendation overread,
evidence-packet truthfulness overclaim, and compatibility-as-migration-approval.
`SCEN-PF-04` is mitigated by the machine-readable adoption gate matrix;
`SCEN-PF-01`, `SCEN-PF-02`, and `SCEN-PF-03` remain open.
