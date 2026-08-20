# Requirements

## Scope

SCENARIUM public Rust contract and first two adoption paths.

## Requirement Table

| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | Identical scenario identity and seed input shall produce identical pseudo-random sequences and serialized evidence. | Mission: reproducible evidence | Replay is the minimum trust property. | must | Simulation Auditor | deterministic tests | accepted |
| REQ-002 | A run shall identify its scenario, adapter, seed, and typed variant, including inertia as a first-class baseline. | VAL-001/003 | SIGNALS requires inertia visibility; consumers need stable identity. | must | Decision Skeptic | API tests and inspection | accepted |
| REQ-003 | Comparisons shall evaluate the complete shared metric set and reject missing metrics, incompatible directions, non-finite values, or scenario mismatches. | VAL-001/003 | Selective or incomparable metrics must not yield success-shaped reports. | must | Adversarial Evidence Reviewer | negative tests | accepted |
| REQ-004 | Comparisons shall preserve improved, regressed, mixed, and equivalent outcomes without collapsing them into a recommendation. | Mission: inspectable decisions | Consumer policy, not SCENARIUM, decides what to do. | must | Decision Skeptic | table-driven tests | accepted |
| REQ-005 | Findings and validation failures shall be structured, serializable, and distinguish notes, warnings, and errors. | Evidence review | Callers need machine-readable failure posture. | must | Evidence Custodian | API and serialization tests | accepted |
| REQ-006 | Evidence packets shall carry caller-supplied provenance and reject silent artifact replacement or duplicate evidence identities. | VAL-002 | SIGNALS append-only provenance must survive interchange. | must | Evidence Custodian | negative tests | accepted |
| REQ-007 | Public serialized records shall declare a schema version and have a documented compatibility and migration policy. | Ecosystem release | Rust type compatibility alone does not protect stored evidence. | must | API Stability Reviewer | schema fixtures and review | proposed |
| REQ-008 | RALLY compatibility shall be proven before its neutral run/comparison/finding/packet types are removed; game mechanics shall remain in RALLY. | VAL-001 | Extraction must preserve a healthy owner system. | must | Runtime Boundary Engineer | compatibility fixture | accepted |
| REQ-009 | A SIGNALS adapter shall map one existing JSON sidecar and provenance block without importing skills, rubrics, campaigns, or recommendation policy. | VAL-002 | Reuse evidence, not methodology ownership. | should | Runtime Boundary Engineer | fixture round-trip | accepted |
| REQ-010 | One non-game adopter shall use SCENARIUM directly and demonstrate deterministic comparison plus measurable neutral-code deletion. | VAL-003 | A second adopter is required before generalization. | must | Consumer Advocate | integration test and diff analysis | accepted |
| REQ-011 | The crate shall document MSRV, dependency policy, feature policy, and semantic-versioning rules before crates.io publication. | Ecosystem release | Adoption requires predictable maintenance. | must | Rust Ecosystem Maintainer | package inspection | proposed |
| REQ-012 | Evidence integrity shall support optional caller-provided digests and detect contradictory or incomplete packet references. | Adversarial evidence review | Provenance labels alone do not prove artifact integrity. | should | Adversarial Evidence Reviewer | validation tests | proposed |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need or validation scenario.
- [x] Each requirement avoids implementation detail unless required for compatibility.

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| REQ-D-001: `no_std` support | Current adopters use `std`, maps, and JSON; no measured embedded need. | A named adopter cannot use SCENARIUM with `std`. |
| REQ-D-002: statistical significance tests | Metric interpretation belongs to consumers until two adopters require the same method. | Two consumers implement equivalent significance logic. |
| REQ-D-003: runtime orchestration | SCENARIUM records evidence but does not execute scenarios. | A separate runtime owner requests a stable adapter contract. |

