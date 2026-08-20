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
| Runtime Boundary Engineer | pass_with_risk | Owner boundaries are strong, but adapter placement must remain consumer-local until two compatible adapters exist. |
| Simulation Auditor | pass_with_risk | Determinism exists, but seed identity collisions and modulo-biased bounded sampling are not yet specified or resolved. |
| Decision Skeptic | blocked_for_adoption | Empty metric sets can report `Equivalent`, and the candidate input is not required to be a candidate variant. |
| Evidence Custodian | blocked_for_adoption | Public fields and derived deserialization can create invalid records without constructor checks; packet reference integrity is incomplete. |
| Consumer Advocate | pass_with_risk | The deletion target is sound, but WP-004 needs a quantitative simplification threshold. |
| API Stability Reviewer | blocked_for_adoption | Persisted records are unversioned and direct public-field mutation makes invariants difficult to preserve compatibly. |
| Rust Ecosystem Maintainer | pass_with_risk | One crate is appropriate; the monolithic source, MSRV, and RNG contract need closure before release. |
| Adversarial Evidence Reviewer | blocked_for_adoption | Target metrics can silently use zero, delta arithmetic can become non-finite, and incomplete comparisons can look trustworthy. |

## Role Review Matrix

| Lane | Required | Reviewer / Role | Decision | Evidence / Rationale |
|---|---|---|---|---|
| Systems engineering | yes | Runtime Boundary Engineer | pass_with_risk | Owner boundaries are explicit; adapters remain unproven. |
| Requirements traceability | yes | API Stability Reviewer | pass_with_risk | Requirements/spec/work packages map; schema version is missing. |
| V&V | yes | Simulation Auditor | pass_with_risk | Foundation tests pass; L2 consumer scenarios remain pending. |
| Software assurance | yes | Rust Ecosystem Maintainer | pass_with_risk | CI/package foundation exists; MSRV and release policy are pending. |
| Security/privacy | yes | Adversarial Evidence Reviewer | pass_with_risk | No secrets/runtime access; misleading or incomplete packet cases need tests. |
| Safety/mission impact | no | Decision Skeptic | not_required | Crate does not make operational decisions; it must preserve ambiguity. |
| Source custody | no | Evidence Custodian | not_required | SCENARIUM stores references/provenance, not source corpora. |
| Configuration/change control | yes | API Stability Reviewer | pass_with_risk | Semver exists; JSON schema change control is not yet explicit. |

## Evidence Inspected

- SCENARIUM README, product plan, source, tests, CI, and package output.
- RALLY README and neutral run/report/evidence types.
- SIGNALS README and append-only/provenance principles.
- VTRACE adoption guidance and templates.

## Findings

| ID | Severity | Finding | Required Action | Disposition |
|---|---|---|---|---|
| FIND-001 | major | Persisted SCENARIUM records do not declare a schema version. | Add versioned document envelopes and retained fixtures in WP-001. | open |
| FIND-002 | major | Packet validation prevents duplicate artifact names but not missing references, invalid digests, or incomplete packet claims. | Add explicit packet validation and adversarial tests in WP-001. | open |
| FIND-003 | major | RALLY extraction is a design thesis, not compatibility evidence. | Complete WP-002 before migration. | open |
| FIND-004 | minor | The foundation is implemented in one large `lib.rs`, weakening focused review. | Split internal modules during WP-001 without splitting the crate. | open |
| FIND-005 | minor | MSRV, feature, and schema compatibility policies are absent. | Complete WP-005 before crates.io publication. | open |
| FIND-006 | note | SIGNALS interchange has a clear boundary but no retained fixture. | Complete WP-003. | open |
| FIND-007 | major | `RunRecord::run_id` omits `seed_label`, and `Seed::from_u64(0)` aliases seed `1`; distinct evidence runs can share identity or random sequence. | Define collision-resistant run identity and preserve distinct supported seeds in WP-001. | open |
| FIND-008 | major | `Metric::new(..., MetricDirection::Target)` creates a target metric without a target, and comparison silently substitutes `0.0`. | Make invalid target construction impossible or reject it; remove the silent fallback. | open |
| FIND-009 | major | `compare_runs` accepts an inertia or named baseline as the candidate and reports an empty shared metric set as `Equivalent`. | Require a candidate variant and at least one comparable metric. | open |
| FIND-010 | major | Finite input metrics can overflow subtraction and produce a non-finite `beneficial_change`. | Check derived deltas and return a structured arithmetic error. | open |
| FIND-011 | major | Derived `Deserialize` and public mutable fields bypass constructor invariants for scenarios, variants, metrics, provenance, runs, and packets. | Add validated document decoding and a public `validate` path; decide which fields remain directly mutable before schema v1. | open |
| FIND-012 | minor | `next_bounded` uses modulo reduction without documenting statistical bias. | Use rejection sampling or explicitly constrain the RNG contract to non-statistical fixture selection. | open |
| FIND-013 | minor | Packet run/comparison vectors preserve insertion order and can reference comparisons without their runs, so logically equivalent packets may serialize differently or incompletely. | Canonicalize packet membership and validate reference closure. | open |
| FIND-014 | minor | “Net code deletion” has no counting rule, allowing adoption success to be claimed selectively. | Define deletion as neutral production LOC removed exceeding adopter glue LOC added, excluding fixtures and generated files. | open |

## Accepted Risks

| Risk | Rationale | Owner | Revisit Trigger |
|---|---|---|---|
| One crate and one source file at foundation. | Avoid premature package design; WP-001 must modularize internally. | Rust Ecosystem Maintainer | Any additional public capability. |
| `std` and serde dependency. | Current named users require JSON and allocated records. | Rust Ecosystem Maintainer | A blocked named `no_std` adopter. |

## Required Follow-Up

1. Execute WP-001 and close FIND-001, FIND-002, FIND-004, and FIND-007 through FIND-013.
2. Re-run the readiness gate before WP-002 migration.
3. Do not publish to crates.io before WP-004 and WP-005 closure.

## Result

The mission, requirements, specification, and package sequence are coherent.
Core hardening is ready to implement. The current crate is not adoption-ready:
the blocked-for-adoption findings above must close before WP-002 or WP-003 can
change an owner system.
