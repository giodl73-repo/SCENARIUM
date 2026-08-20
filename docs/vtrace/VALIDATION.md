# Validation Plan

## Scope

Validate SCENARIUM against intended owner and adopter workflows, not only unit
behavior.

## Validation Scenarios

| Scenario ID | User / Actor | Need | Workflow | Success Criteria | Evidence Pointer | Result |
|---|---|---|---|---|---|---|
| VAL-001 | RALLY maintainer | Reuse neutral evidence without losing games behavior. | Map representative RALLY run, comparison, finding, and packet fixtures into SCENARIUM; run both suites. | Accepted and failure behavior is equivalent; game mechanics remain in RALLY; exact deletion plan exists. | EVID-RALLY-001 | passed |
| VAL-002 | SIGNALS maintainer / evidence reviewer | Exchange append-only artifacts with provenance. | Convert one real JSON sidecar/frontmatter record into SCENARIUM provenance and packet form and back. | Required provenance survives; unknown fields are visible; no skills or recommendation policy enter SCENARIUM. | EVID-SIGNALS-001 | passed |
| VAL-003 | Non-game simulation owner | Compare inertia and a candidate reproducibly. | Adopt SCENARIUM in CERES or one repeated enterprise simulation harness. | Repeated output is identical; consumer metrics stay local; neutral-code deletion exceeds adapter code added. | EVID-ADOPT-001 | pending |
| VAL-004 | Adversarial reviewer | Detect trustworthy-looking but incomplete evidence. | Submit empty metrics, baseline-as-candidate runs, targetless metrics, overflow deltas, invalid deserialized records, seed collisions, missing references, duplicate artifacts, invalid digests, and mixed outcomes. | No invalid case returns a successful complete packet or recommendation-shaped result. | EVID-ADV-001 | pending |
| VAL-005 | Rust ecosystem maintainer | Depend on and upgrade the crate predictably. | Package, document, and test retained schema fixtures across a minor-version simulation. | MSRV/features/semver policy is clear and supported fixtures remain readable. | EVID-RELEASE-001 | pending |

## Acceptance Evidence

| Evidence ID | Scenario ID | Evidence | Result |
|---|---|---|---|
| EVID-RALLY-001 | VAL-001 | RALLY commit `6b0bbc6`: retained compatibility fixture, local adapter, three integration tests, and exact deletion ledger | passed |
| EVID-SIGNALS-001 | VAL-002 | SIGNALS commit `ea5f090`; retained SCENARIUM fixture and three interchange tests | passed |
| EVID-ADOPT-001 | VAL-003 | adopter commit, tests, before/after LOC and type inventory | pending |
| EVID-ADV-001 | VAL-004 | adversarial test matrix and role review | pending |
| EVID-RELEASE-001 | VAL-005 | package/docs/MSRV/schema compatibility report | pending |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Embedded or `no_std` consumer | No named user. | Future API may assume allocation and JSON. | Named adopter with a blocked use case. |
| Statistical inference library | Consumer methods differ and are policy-laden. | Users may overread raw deltas. | Two adopters request the same test and interpretation boundary. |
| Distributed evidence registry | Outside crate mission. | Packet exchange remains file/API based. | Separate registry owner and protocol proposal. |
