# Work Packages

## Scope

SCENARIUM implementation goals through first ecosystem-ready release.

VTRACE terms remain internal coordination concepts and shall not become public
SCENARIUM commands or schema fields.

## Work Package Table

| ID | Objective | Product Requirement | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|---|---|---|
| WP-001 | Harden the core contract. | Versioned deterministic evidence with explicit failure behavior. | REQ-001..007/011..016; SPEC-001..005/009/010; IF-001/002; CR-001..010 | SCENARIUM `src/`, tests, README | Foundation green; spec gate passed with risk. | Schema version, packet validation, golden fixtures, modular source, all checks green. | L0 yes / L1 yes / L2 no | complete |
| WP-002 | Prove RALLY compatibility. | Neutral RALLY evidence maps without moving mechanics. | REQ-008; SPEC-006; IF-003 | RALLY adapter/fixtures, SCENARIUM tests | WP-001 L1 complete. | Accepted and failure fixtures pass; migration/deletion plan reviewed. | L0 yes / L1 yes / L2 yes | complete |
| WP-003 | Prove SIGNALS artifact interchange. | One sidecar maps losslessly with provenance. | REQ-006/009; SPEC-004/007; IF-004 | SIGNALS fixture/adapter, SCENARIUM tests | WP-001 L1 complete. | Known fields round-trip; unknowns observable; no methodology dependency. | L0 yes / L1 yes / L2 yes | complete |
| WP-004 | Prove a non-game adopter. | Reuse the crate and delete duplicated neutral code. | REQ-010; SPEC-008 | Selected consumer and SCENARIUM fixture | WP-002 or WP-003 L2 complete; adopter selected. | Deterministic run/comparison passes and net deletion is recorded. | L0 yes / L1 yes / L2 yes | complete |
| WP-005 | Establish release readiness. | Publish a stable documented crate contract. | REQ-007/011; SPEC-005; IF-001/002 | Cargo metadata, docs, schemas, fixtures | WP-004 L2 complete. | MSRV/semver/features documented; package and compatibility gates pass. | L0 yes / L1 yes / L2 yes | complete |

## WP-001: Core contract hardening

Product files/modules to edit:

- Split `src/lib.rs` into focused internal modules while retaining one crate.
- Add schema version fields or versioned document envelopes.
- Add packet validation for missing references and optional digests.
- Add golden JSON fixtures and complete comparison tables.
- Define collision-resistant run identity including seed identity.
- Remove zero-seed aliasing and define bounded RNG bias guarantees.
- Prevent target metrics without targets and reject non-finite derived deltas.
- Require candidate variants and at least one comparable metric.
- Validate deserialized/publicly constructed records before use.
- Canonicalize packet membership and enforce run/comparison reference closure.
- Update README compatibility policy.

Exit criteria:

- No public persisted record lacks a versioning strategy.
- All comparison statuses and invalid-input classes have tests.
- Identical inputs produce byte-identical fixture output.
- No silent artifact replacement, missing metric, or missing reference succeeds.
- Distinct supported seeds and seed labels cannot silently collapse to one run identity.
- Empty comparisons, baseline-as-candidate comparisons, targetless metrics, and
  non-finite derived deltas return structured errors.
- Deserialized records pass the same invariants as constructor-created records.
- FIND-001, FIND-002, FIND-004, and FIND-007 through FIND-013 are closed.

Review gate: API Stability Reviewer, Simulation Auditor, Adversarial Evidence Reviewer.

## WP-002: RALLY compatibility

Product files/modules to edit:

- RALLY-local compatibility fixture and adapter.
- SCENARIUM fixture tests only if a neutral seam is proven.
- RALLY dependency and neutral-type deletion in a separate migration commit.

Forbidden changes:

- Moving dice, turn order, board/card primitives, hidden zones, grid helpers, or
  game/playtest policy into SCENARIUM.

Exit criteria:

- Current RALLY accepted outputs and structured failures have equivalents.
- Any incompatibility is documented and explicitly accepted.
- Deletion list names exact RALLY types and lines removed.

Review gate: Runtime Boundary Engineer, Consumer Advocate, Simulation Auditor.

## WP-003: SIGNALS evidence bridge

Product files/modules to edit:

- One retained SIGNALS JSON sidecar fixture.
- A consumer-local conversion function or adapter.
- Round-trip and unknown-field behavior tests.

Forbidden changes:

- Importing SIGNALS skills, rubrics, campaign sequencing, personas, or automatic
  decision recommendations.

Exit criteria:

- Skill/topic/item/date/version/input provenance survives mapping.
- Append-only artifact identity is preserved.
- Unknown fields are retained or reported, never silently dropped.

Review gate: Evidence Custodian, Runtime Boundary Engineer, Decision Skeptic.

## WP-004: Non-game adoption

Adopter selection rule:

- Prefer an existing repeated simulation harness such as CERES or one enterprise
  `*-sim` crate.
- Select the adopter with the clearest duplicated neutral types and smallest
  product-policy surface.

Exit criteria:

- Consumer uses SCENARIUM directly.
- Consumer metrics and policy remain local.
- Before/after deletion ledger shows neutral production LOC removed exceeds
  adopter glue LOC added, excluding tests, fixtures, generated files, and docs.
- At least one duplicated neutral public type family is removed.
- No new general API is added solely for one adopter.

Review gate: Consumer Advocate, Rust Ecosystem Maintainer, Runtime Boundary Engineer.

## WP-005: Release readiness

Exit criteria:

- MSRV and support policy documented.
- Semver and JSON schema compatibility policy documented.
- `cargo package` and docs build cleanly.
- Supported schema fixtures are retained.
- Registry publication remains prohibited by portfolio policy.

Review gate: API Stability Reviewer, Rust Ecosystem Maintainer, Evidence Custodian.

## Orphan Check

- [x] Every accepted requirement is assigned or deferred.
- [x] Every specification is assigned to a work package.
- [x] Interface-changing packages name interface IDs.
- [x] Critical work names code-rigor constraints.
- [x] Every package has exit criteria and validation levels.
- [x] No package is process-only cleanup.
