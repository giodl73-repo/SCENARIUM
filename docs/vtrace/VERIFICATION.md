# Verification Plan

## Scope

SCENARIUM core, compatibility adapters, and first adoption.

## Verification Matrix

| Requirement IDs | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001..006 | test/analysis | `cargo test` | Determinism, comparison, failure, and packet tests pass. | current pass | EVID-001..006 |
| REQ-007/011 | inspection/test | golden fixtures, `cargo package`, docs build, Rust 1.74 lane | Versioned schema and release policy are complete. | passed | EVID-API-001/EVID-RELEASE-001 |
| REQ-008 | integration | SCENARIUM and RALLY tests with compatibility fixture | Equivalent accepted/failure behavior. | passed at RALLY `6b0bbc6` | EVID-RALLY-001 |
| REQ-009 | round-trip | SIGNALS fixture adapter test | Provenance and unknown-field posture preserved. | passed with SIGNALS `ea5f090` | EVID-SIGNALS-001 |
| REQ-010 | integration/analysis | adopter suite and deletion diff | Second adopter passes with net simplification. | passed at CERES `71c7ef2` | EVID-ADOPT-001 |
| REQ-012 | adversarial test | incomplete/digest/duplicate/selective packet cases | Invalid evidence is rejected or reported. | passed | EVID-ADV-001 |
| CR-001..010 | static analysis/review | fmt, Clippy, tests, source and manifest review | Code-rigor constraints pass or have accepted findings. | passed | EVID-ADV-001/EVID-RELEASE-001 |

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
| L0 | Active package sanity. | fmt plus focused tests | passed |
| L1 | Full SCENARIUM confidence. | all standard commands and package check | passed |
| L2 | Migration or release readiness. | consumer suites, fixtures, role gate, deletion evidence | passed |

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
| RALLY consumer migration not yet executed. | Recorded consumers remain pinned to RALLY neutral types. | Consumer-specific migration commits |
| No production SIGNALS runtime integration. | Interchange is fixture-proven, not a runtime dependency. | Revisit only for a named runtime consumer. |
| Only one non-game adopter. | Generality beyond CERES remains bounded. | Add adopters only for named product need. |
