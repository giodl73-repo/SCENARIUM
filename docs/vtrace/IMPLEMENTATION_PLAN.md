# Implementation Plan

## Scope

SCENARIUM contract hardening and first adoption proofs.

Implementation baseline: `SPECIFICATION_BASELINE.md`, decision `pass_with_risk`.

## Baseline Inputs

| Artifact | Status | Notes |
|---|---|---|
| `MISSION.md` | accepted | Owner-first mission and deletion target defined. |
| `REQUIREMENTS.md` | accepted | Twelve requirements plus three deferrals. |
| `SPECIFICATION_BASELINE.md` | accepted | Mixed current/target baseline. |
| `CODE_RIGOR.md` | accepted | Medium-risk Rust constraints. |
| `VERIFICATION.md` | accepted | L0/L1/L2 commands and evidence planned. |
| `VALIDATION.md` | accepted | RALLY, SIGNALS, and non-game scenarios. |

## Implementation Strategy

Harden the existing single crate before migration. Add explicit schema and
packet validation, then prove compatibility at owner boundaries. Keep adapters
consumer-local until repeated use demonstrates a stable shared seam. Delete
duplicated owner code only after L2 evidence.

## Sequencing

| Order | Product Capability | Product Surfaces To Edit | Work Package | Why This Order |
|---:|---|---|---|---|
| 1 | Versioned, reviewable core contract | SCENARIUM `src/`, tests, docs | WP-001 | Adoption cannot safely target an unstable schema. |
| 2 | RALLY compatibility | RALLY fixture/adapter plus SCENARIUM tests | WP-002 | Closest owner and largest deletion target. |
| 3 | SIGNALS provenance interchange | SIGNALS sidecar fixture and adapter | WP-003 | Proves non-runtime artifact use and provenance. |
| 4 | Non-game direct adoption | Selected consumer plus SCENARIUM fixture | WP-004 | Proves generality before adding adapters. |
| 5 | Ecosystem release contract | Cargo/docs/schema fixtures | WP-005 | Publish only after real adoption evidence. |

## Source-To-Work-Package Mapping

| Source IDs | Work Package | Disposition | Notes |
|---|---|---|---|
| REQ-001..007/012, SPEC-001..005 | WP-001 | implement | Core hardening. |
| REQ-008, SPEC-006, IF-003 | WP-002 | implement | RALLY proof. |
| REQ-009, SPEC-007, IF-004 | WP-003 | implement | SIGNALS proof. |
| REQ-010, SPEC-008 | WP-004 | implement | Second adopter and deletion measurement. |
| REQ-011, SPEC-005, IF-001/002 | WP-005 | implement | Release readiness. |
| REQ-D-001..003 | deferred | defer | Trigger-based only. |

## Branch / Change Control

Branch strategy: one branch per work package after foundation.

Worktree strategy: use a separate worktree when a package touches SCENARIUM and
an owner repo.

Change-control trigger: any public Rust type, JSON field, metric semantics,
error behavior, or adapter ownership change.

Rollback strategy: revert the consumer migration while retaining compatibility
fixtures; never rewrite stored evidence silently.

## Wave / Pulse Policy

Active wave: `2026-08-19-foundation`

Pulse mapping:

- Pulse 02 -> WP-002
- Pulse 03 -> WP-003
- Pulse 04 -> WP-004
- WP-001 precedes Pulse 02 and may be recorded as a new hardening pulse.
- WP-005 begins only after WP-004 L2 closure.

## Product / Process / Verification Split

| Work Package | Product Requirement | Implementation Area | Verification Command | VTRACE-Only Closeout |
|---|---|---|---|---|
| WP-001 | Versioned deterministic evidence contract. | SCENARIUM crate. | fmt/clippy/test/package plus golden fixtures. | evidence, trace, review rows |
| WP-002 | RALLY-compatible neutral evidence types. | RALLY adapter/tests and SCENARIUM fixtures. | both repo test suites. | compatibility decision |
| WP-003 | SIGNALS provenance interchange. | Adapter/fixture only. | round-trip test. | evidence and boundary review |
| WP-004 | Non-game reusable contract. | Selected adopter. | adopter plus SCENARIUM tests. | deletion ledger |
| WP-005 | Stable ecosystem release surface. | Cargo/docs/schema policy. | package/docs/compatibility tests. | readiness review |

## Validation Levels

| Level | Scope | Required Commands / Evidence | Required Before |
|---|---|---|---|
| L0 | Active code sanity | `cargo fmt --check`, focused tests | commit |
| L1 | Full SCENARIUM confidence | Clippy, all tests, docs, package | push/PR |
| L2 | Owner/adopter readiness | both repo suites, compatibility fixture, role review, deletion evidence | migration/release |

## Risks

| Risk ID | Risk | Mitigation | Owner |
|---|---|---|---|
| RISK-001 | SCENARIUM duplicates RALLY rather than simplifying it. | Require deletion measurement and delayed migration. | Consumer Advocate |
| RISK-002 | SIGNALS methodology leaks into a low-level crate. | Adapter-only boundary and runtime review. | Runtime Boundary Engineer |
| RISK-003 | Serialized evidence changes without migration. | Schema version, golden fixtures, API stability review. | API Stability Reviewer |
| RISK-004 | Selective metrics create misleading success. | Complete-set comparison and adversarial tests. | Adversarial Evidence Reviewer |
| RISK-005 | Premature crate/module proliferation. | Keep one published crate through two adopters. | Rust Ecosystem Maintainer |

## Implementation Readiness Decision

Decision: pass_with_risk

Rationale: WP-001 is ready. WP-002 through WP-005 remain gated by the preceding
package and their L2 evidence.

