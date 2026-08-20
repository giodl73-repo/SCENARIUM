# Specification Baseline

## Scope

Repo: SCENARIUM

Baseline type: mixed (`current` foundation plus `target` adoption contract)

Baseline date: 2026-08-19

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| README and product plan | `README.md`, `PRODUCT_PLAN.md` | current | Product boundary and deletion target. |
| Foundation tests | `tests/contract.rs` | current | Ten integration tests cover replay, identity, comparisons, failures, decoding, and packets. |
| Public API | `src/*.rs` | current | One modular crate with versioned validated records. |
| RALLY behavior | RALLY `src/lib.rs` | current owner | Compatibility source, not copied authority. |
| SIGNALS artifacts | SIGNALS `PRINCIPLES.md` | current owner | Provenance and append-only design evidence. |
| Downstream adoption | none | unknown | No SCENARIUM consumer yet. |

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | State | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001/002 | software | current | `Scenario`, `Seed`, `RunVariant`, and `RunRecord` provide deterministic identity and execution inputs. | unit test | VAL-001/003 | Simulation Auditor | medium | implemented |
| SPEC-002 | REQ-003/004 | software | current | `compare_runs` rejects incompatible inputs and emits directional deltas with four observable statuses. | table/negative tests | VAL-001/003 | Decision Skeptic | high | implemented |
| SPEC-003 | REQ-005 | software | current | Findings and run status distinguish note, warning, error, pass, review, and error. | unit test | reviewer inspection | Evidence Custodian | medium | verified |
| SPEC-004 | REQ-006/012 | package/interface | current | Evidence packets are append-only, canonical, digest-validated, and closed over referenced runs. | unit/fixture tests | VAL-002 | Evidence Custodian | high | verified |
| SPEC-005 | REQ-007/011 | package/interface | current | Every persisted document declares schema version; release docs declare MSRV, features, dependencies, and compatibility policy. | fixture/package inspection | ecosystem review | API Stability Reviewer | high | verified |
| SPEC-006 | REQ-008 | interface/test | target | A RALLY fixture maps neutral types both directions or documents bounded incompatibilities before migration. | compatibility test | VAL-001 | Runtime Boundary Engineer | high | proposed |
| SPEC-007 | REQ-009 | interface/test | target | A SIGNALS sidecar maps to SCENARIUM provenance/evidence without importing SIGNALS policy. | round-trip fixture | VAL-002 | Runtime Boundary Engineer | medium | proposed |
| SPEC-008 | REQ-010 | integration/test | target | A non-game consumer adopts SCENARIUM and records net neutral-code deletion. | integration test/diff | VAL-003 | Consumer Advocate | high | proposed |
| SPEC-009 | REQ-013/014 | software/interface | current | All construction and decoding paths enforce shared invariants, and run identity includes seed-bearing inputs without supported-seed aliasing. | negative/collision tests | VAL-001/004 | Evidence Custodian | high | verified |
| SPEC-010 | REQ-015/016 | software | current | Comparison rejects empty, mis-typed, targetless, or non-finite cases; bounded RNG uses rejection sampling. | boundary/algorithm tests | VAL-003/004 | Simulation Auditor | high | verified |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001/002/003 | Rust API | Semver; no silent semantic change to direction, status, or error behavior. | Public type or method change. | API tests |
| IF-002 | SPEC-004/005 | JSON records | Schema version required before 0.2; fixtures retained for every supported version. | Field add/remove/rename or enum change. | golden fixtures |
| IF-003 | SPEC-006 | RALLY adapter | Preserve current RALLY accepted and failure behavior; mechanics excluded. | RALLY neutral type migration. | compatibility fixture |
| IF-004 | SPEC-007 | SIGNALS adapter | Preserve source provenance fields and artifact identity; unknown fields remain observable. | Sidecar schema mapping change. | round-trip fixture |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001..005/009/010 | `scenarium` crate | Neutral records, validation, comparison, serialization, and deterministic sampling. | Consumer execution, metrics, recommendation, workflow. | L0/L1 |
| SPEC-006 | RALLY-local adapter/test | RALLY compatibility and migration evidence. | Moving game mechanics into SCENARIUM. | L2 |
| SPEC-007 | SIGNALS-local or SCENARIUM adapter fixture | Artifact interchange proof. | SIGNALS skills/rubrics/campaign logic. | L2 |
| SPEC-008 | adopter-owned integration | Real reuse and deletion evidence. | New generic API before repeated need. | L2 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-001..005 | Determinism | Identical inputs produce byte-identical JSON. | repeated golden test | proposed |
| SPEC-NF-002 | SPEC-001..005 | Dependency restraint | No dependency added without a named capability and consumer. | manifest review | accepted |
| SPEC-NF-003 | SPEC-002/004 | Failure visibility | Invalid or incomplete evidence returns structured error; no success fallback. | negative tests | accepted |
| SPEC-NF-004 | SPEC-005 | Compatibility | Supported schema fixtures remain readable across minor releases. | compatibility test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | First non-game adopter selection. | Blocks L2 adoption proof. | Decide during WP-004; prefer CERES or an enterprise simulation with repeated shapes. | Consumer Advocate |
| SPEC-UNK-002 | Whether adapters belong in SCENARIUM or consumers. | Could expand scope prematurely. | Keep consumer-local through first two proofs. | Runtime Boundary Engineer |
| SPEC-UNK-003 | MSRV value. | Blocks release policy. | Measure current dependency MSRV in WP-005. | Rust Ecosystem Maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001/002 | SPEC-001 | covered | Foundation implemented. |
| REQ-003/004 | SPEC-002 | covered | Add fuller table coverage in WP-001. |
| REQ-005 | SPEC-003 | covered | Finding status behavior verified. |
| REQ-006/012 | SPEC-004 | covered | Canonical packet closure, duplicate rejection, and digest validation verified. |
| REQ-007/011 | SPEC-005 | covered | Schema, fixture, MSRV, semver, feature, and dependency policy verified. |
| REQ-008 | SPEC-006 | covered | Implementation pending. |
| REQ-009 | SPEC-007 | covered | Implementation pending. |
| REQ-010 | SPEC-008 | covered | Adopter selection pending. |
| REQ-013/014 | SPEC-009 | covered | Validated decoding and collision tests pass. |
| REQ-015/016 | SPEC-010 | covered | Strict comparison and rejection-sampling tests pass. |

## Specification Gate

Decision: pass_with_risk

Required before implementation:

- [x] Accepted requirements map to specifications or deferrals.
- [x] Work packages name parent specification IDs.
- [x] Public contracts have owners and change triggers.
- [x] Unknowns have explicit discovery or defer dispositions.
- [x] Verification and validation methods are credible.

Rationale: contract hardening may begin. Consumer migration is blocked until
the relevant compatibility work package reaches L2.
