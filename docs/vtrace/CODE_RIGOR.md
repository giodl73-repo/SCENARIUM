# Code Rigor

## Scope

SCENARIUM public crate and adapters.

Risk level: medium

Language/toolchain: stable Rust 2021, rustfmt, Clippy, cargo test, cargo package.

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| CR-001 | Public behavior shall be implemented in focused modules; functions have a 60 logical-line soft cap. | Hand-authored crate code | review and size inspection | Larger units require rationale and focused tests. |
| CR-002 | Comparison and validation branches shall have table-driven accepted and failure tests. | Critical decision logic | tests | No waiver for public comparison states. |
| CR-003 | Public interfaces shall reject invalid input explicitly; no silent omission, coercion, or success fallback. | APIs and schemas | negative tests | Impossible states require documented proof. |
| CR-004 | Determinism, complete metric comparison, append-only packets, and schema compatibility shall have invariant tests. | Critical contracts | unit/golden/property tests | Waiver requires role review. |
| CR-005 | Formatting, Clippy warnings, tests, docs, and package verification shall be clean. | Whole crate | standard commands | Toolchain regressions require an issue and pinned workaround. |
| CR-006 | `unsafe` code is forbidden in the foundation and first adoption waves. | Whole crate | source inspection | Requires a separate design gate. |
| CR-007 | Public serialized enums and fields require change-control review and retained fixtures. | JSON surfaces | diff/fixture review | None after schema v1. |
| CR-008 | Dependencies require a named product capability, license check, and deletion/maintenance rationale. | Cargo manifest | manifest review | Dev-only test dependencies may use a lighter review. |

## Tailoring

| Area | Rule | Rationale |
|---|---|---|
| Module layout | Split the current monolithic `lib.rs` during WP-001 without splitting the published crate. | Improves reviewability without premature package boundaries. |
| Errors | Keep a typed error enum; add context without string matching as a contract. | Consumers need structured failures. |
| Serialization | Use ordered collections and golden fixtures. | Identical inputs must remain byte-stable. |
| Numeric values | Reject NaN and infinity; document float comparison semantics. | Non-finite evidence is not portable or trustworthy. |

## Exceptions / Waivers

None accepted.

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| EVID-CR-001 | CR-005/006 | `cargo fmt --check; cargo clippy --all-targets -- -D warnings; cargo test` and source inspection | current pass | CI and local run |
| EVID-CR-002 | CR-001 | Current `src/lib.rs` review | risk accepted for foundation only | FIND-004 in `REVIEW.md` |

