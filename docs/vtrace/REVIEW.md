# Review Gate

## Scope

SCENARIUM specification and implementation readiness.

Gate type: specification/readiness

Decision: pass_with_risk

Date: 2026-08-19

Reviewer lenses: SCENARIUM `.roles/ROLE.md`

## SCENARIUM Role Findings

| Role | Decision | Finding |
|---|---|---|
| Runtime Boundary Engineer | pass | RALLY compatibility and SIGNALS interchange are proven without moving owner mechanics or methodology. |
| Simulation Auditor | pass | Run identity includes seed-bearing inputs, integer seeds remain distinct, and bounded sampling uses rejection sampling. |
| Decision Skeptic | pass | Empty comparisons and baseline-as-candidate inputs fail; all four valid outcomes remain visible. |
| Evidence Custodian | pass | Packet closure is canonical and real SIGNALS provenance plus unknown extension fields round-trip losslessly. |
| Consumer Advocate | pass_with_risk | RALLY has an exact consumer-gated deletion ledger; no non-game deletion evidence exists yet. |
| API Stability Reviewer | pass | `scenarium.v1`, retained fixtures, MSRV, semver, and compatibility rules are explicit. |
| Rust Ecosystem Maintainer | pass | One crate remains, code is modular, MSRV is declared, and dependency/feature policy is documented. |
| Adversarial Evidence Reviewer | pass | Targetless, empty, mis-typed, overflow, invalid-digest, replacement, and missing-reference cases fail structurally. |

## Role Review Matrix

| Lane | Required | Reviewer / Role | Decision | Evidence / Rationale |
|---|---|---|---|---|
| Systems engineering | yes | Runtime Boundary Engineer | pass | Core ownership is explicit and the RALLY adapter leaves dice, turns, board state, events, and policy in RALLY. |
| Requirements traceability | yes | API Stability Reviewer | pass | WP-001 through WP-003 requirements, specifications, fixtures, commits, and evidence pointers map completely. |
| V&V | yes | Simulation Auditor | pass | Core, RALLY, and SIGNALS tests cover accepted, failure, comparison, provenance, schema, and unknown-field behavior. |
| Software assurance | yes | Rust Ecosystem Maintainer | pass | Modular code, Rust 1.74, clean package policy, and standard Rust gates are established. |
| Security/privacy | yes | Adversarial Evidence Reviewer | pass | Invariant-safe decoding, digest validation, canonical closure, and strict errors prevent success-shaped invalid evidence. |
| Safety/mission impact | no | Decision Skeptic | not_required | Crate does not make operational decisions; it must preserve ambiguity. |
| Source custody | yes | Evidence Custodian | pass | The retained SIGNALS sidecar identifies its source artifact and commit without copying methodology or source corpora into records. |
| Configuration/change control | yes | API Stability Reviewer | pass | Semver and `scenarium.v1` change-control rules plus retained fixtures are explicit. |

## Evidence Inspected

- SCENARIUM README, product plan, source, tests, CI, and package output.
- RALLY README and neutral run/report/evidence types.
- SIGNALS README and append-only/provenance principles.
- VTRACE adoption guidance and templates.

## Findings

| ID | Severity | Finding | Required Action | Disposition |
|---|---|---|---|---|
| FIND-001 | major | Persisted SCENARIUM records do not declare a schema version. | Added `scenarium.v1` envelopes and a retained round-trip fixture. | fixed |
| FIND-002 | major | Packet validation prevents duplicate artifact names but not missing references, invalid digests, or incomplete packet claims. | Added digest validation, canonical membership, and reference-closure checks. | fixed |
| FIND-003 | major | RALLY extraction is a design thesis, not compatibility evidence. | RALLY commit `6b0bbc6` adds retained accepted/failure mappings, explicit stricter failures, full-suite proof, and an exact consumer-gated deletion ledger. | fixed |
| FIND-004 | minor | The foundation is implemented in one large `lib.rs`, weakening focused review. | Split into document, error, seed, model, compare, and evidence modules. | fixed |
| FIND-005 | minor | MSRV, feature, and schema compatibility policies are absent. | Declared Rust 1.74 and documented semver, schema, feature, and dependency policy. | fixed |
| FIND-006 | note | SIGNALS interchange has a clear boundary but no retained fixture. | SIGNALS `ea5f090` retains a real sidecar; SCENARIUM round-trips provenance, artifact identity, and unknown extensions in three tests. | fixed |
| FIND-007 | major | `RunRecord::run_id` omits `seed_label`, and `Seed::from_u64(0)` aliases seed `1`; distinct evidence runs can share identity or random sequence. | Run IDs now use injective length-prefixed identity components and integer seeds remain distinct. | fixed |
| FIND-008 | major | `Metric::new(..., MetricDirection::Target)` creates a target metric without a target, and comparison silently substitutes `0.0`. | Target carries its value in the enum and non-finite targets fail. | fixed |
| FIND-009 | major | `compare_runs` accepts an inertia or named baseline as the candidate and reports an empty shared metric set as `Equivalent`. | Candidate and non-empty metric requirements now return typed errors. | fixed |
| FIND-010 | major | Finite input metrics can overflow subtraction and produce a non-finite `beneficial_change`. | Derived deltas are checked and fail with `NonFiniteDerivedMetric`. | fixed |
| FIND-011 | major | Derived `Deserialize` and public mutable fields bypass constructor invariants for scenarios, variants, metrics, provenance, runs, and packets. | Fields are private and serde decoding uses validated `TryFrom` paths inside versioned documents. | fixed |
| FIND-012 | minor | `next_bounded` uses modulo reduction without documenting statistical bias. | Bounded sampling now uses rejection sampling. | fixed |
| FIND-013 | minor | Packet run/comparison vectors preserve insertion order and can reference comparisons without their runs, so logically equivalent packets may serialize differently or incompletely. | BTree collections canonicalize order and comparison insertion requires both runs. | fixed |
| FIND-014 | minor | “Net code deletion” has no counting rule, allowing adoption success to be claimed selectively. | WP-004 now requires removed neutral production LOC to exceed adopter glue LOC, with exclusions defined. | fixed |

## Accepted Risks

| Risk | Rationale | Owner | Revisit Trigger |
|---|---|---|---|
| One crate and one source file at foundation. | Avoid premature package design; WP-001 must modularize internally. | Rust Ecosystem Maintainer | Any additional public capability. |
| `std` and serde dependency. | Current named users require JSON and allocated records. | Rust Ecosystem Maintainer | A blocked named `no_std` adopter. |

## Required Follow-Up

1. Migrate recorded RALLY consumers individually before deleting neutral types.
2. Complete WP-004 with a non-game adopter and net production-code deletion.
3. Do not publish to crates.io before WP-004 and WP-005 closure.

## Result

The mission, requirements, specification, and package sequence are coherent.
WP-001 core hardening, WP-002 RALLY compatibility, and WP-003 SIGNALS
interchange pass their required role lenses. Non-game simplification, owner
migration, and ecosystem release remain gated.
