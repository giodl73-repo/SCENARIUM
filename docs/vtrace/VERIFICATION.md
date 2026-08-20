# Verification Plan

## Scope

SCENARIUM core, compatibility adapters, and first adoption.

## Verification Matrix

| Requirement IDs | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001..006 | test/analysis | `cargo test` | Determinism, comparison, failure, and packet tests pass. | current pass | EVID-001..006 |
| REQ-007/011 | inspection/test | golden fixtures, `cargo package`, docs build | Versioned schema and release policy are complete. | pending | EVID-API-001 |
| REQ-008 | integration | SCENARIUM and RALLY tests with compatibility fixture | Equivalent accepted/failure behavior. | pending | EVID-RALLY-001 |
| REQ-009 | round-trip | SIGNALS fixture adapter test | Provenance and unknown-field posture preserved. | pending | EVID-SIGNALS-001 |
| REQ-010 | integration/analysis | adopter suite and deletion diff | Second adopter passes with net simplification. | pending | EVID-ADOPT-001 |
| REQ-012 | adversarial test | incomplete/digest/duplicate/selective packet cases | Invalid evidence is rejected or reported. | pending | EVID-ADV-001 |
| CR-001..008 | static analysis/review | fmt, Clippy, tests, source and manifest review | Code-rigor constraints pass or have accepted findings. | partial | EVID-CR-001/002 |

## Commands

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --doc
cargo package --allow-dirty
git diff --check
```

VTRACE package:

```powershell
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- .
```

## Validation Levels

| Level | Purpose | Commands / Evidence | Result |
|---|---|---|---|
| L0 | Active package sanity. | fmt plus focused tests | pending per package |
| L1 | Full SCENARIUM confidence. | all standard commands and package check | current foundation pass |
| L2 | Migration or release readiness. | consumer suites, fixtures, role gate, deletion evidence | pending |

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| EVID-001 | test | `tests::seed_is_repeatable` | REQ-001 | pass |
| EVID-002 | test/API | `RunVariant::Inertia`, run construction tests | REQ-002 | pass |
| EVID-003 | tests | comparison and invalid metric tests | REQ-003 | pass/expand |
| EVID-004 | test | improved and mixed status tests | REQ-004 | partial; equivalent/regressed explicit cases pending |
| EVID-005 | API/review | finding/status implementation | REQ-005 | partial; dedicated tests pending |
| EVID-006 | tests | duplicate artifact and deterministic JSON tests | REQ-006 | pass/expand |
| EVID-CI-001 | CI | GitHub Actions run for foundation | REQ-001..006, CR-005 | pass |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| No schema version in persisted records. | Stored evidence can drift silently. | WP-001 |
| No complete RALLY compatibility fixture. | Migration unsafe. | WP-002 |
| No SIGNALS sidecar fixture. | Provenance interchange unproven. | WP-003 |
| No second direct adopter. | Generality and deletion claim unproven. | WP-004 |

