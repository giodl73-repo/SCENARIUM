# SCENARIUM Principles

## SCEN-P-01: Product-Neutral Contract Ownership

**Decision rule:** SCENARIUM may own reusable run, comparison, finding,
provenance, and evidence packet contracts, but consumer repositories own
mechanics, metrics, policy, scoring, orchestration, and release interpretation.

**Rationale:** Moving consumer semantics into the shared crate would reverse
the dependency boundary SCENARIUM was created to protect.

**Test:** Runtime Boundary Engineer review, VTRACE validation, and adopter
deletion ledgers gate new shared APIs.

**Evidence:** `CLAUDE.md`, `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
and `docs/vtrace/MISSION.md`.

## SCEN-P-02: Inertia Must Stay Visible

**Decision rule:** Every comparison keeps inertia or named baselines visible as
typed competitors; SCENARIUM must not collapse mixed, regressed, equivalent, or
improved outcomes into recommendation-shaped summaries.

**Rationale:** The crate reports metric deltas; consumers decide whether those
deltas justify action.

**Test:** Contract tests cover all comparison statuses and Decision Skeptic
review blocks recommendation-shaped API.

**Evidence:** `src/compare.rs`, `src/model.rs`, `tests/contract.rs`,
`docs/vtrace/TRACE.md`, and `.roles/parliament/decision-skeptic.md`.

## SCEN-P-03: Evidence Packets Preserve Lineage, Not Truth

**Decision rule:** Append-only packets preserve run, comparison, artifact, and
provenance lineage, but they do not prove that caller-supplied metrics are
true, complete, or decision-sufficient.

**Rationale:** Structural evidence can still be misleading when the input data
or framing is incomplete.

**Test:** Adversarial evidence review and packet-closure tests keep lineage
claims separate from truthfulness claims.

**Evidence:** `src/evidence.rs`, `tests/contract.rs`,
`docs/vtrace/EVIDENCE.md`, and
`.roles/stakeholders/adversarial-evidence-reviewer.md`.

## SCEN-P-04: Adoption Must Earn Its Dependency

**Decision rule:** Consumer adoption is justified only when compatibility
evidence and deletion ledgers show that duplicated neutral code is reduced
without importing consumer semantics into SCENARIUM.

**Rationale:** Compatibility alone can increase abstraction cost, as BANISH and
TERRAIN stop gates demonstrate.

**Test:** VTRACE adoption scenarios require compatibility evidence, deletion
or stop-gate analysis, and consumer-owned migration review.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/VALIDATION.md`,
`docs/vtrace/EVIDENCE.md`, and `docs/release-readiness.md`.

## SCEN-P-05: Release Evidence Is Exact, Not Ambient

**Decision rule:** Package, docs, lockfile, fixture, CI, and MSRV claims
require the named command or recorded evidence. A nearby stable-toolchain pass
is useful but does not substitute for an explicit Rust 1.74 lane.

**Rationale:** Release-readiness claims become stale when agents cannot
reproduce the exact toolchain evidence they cite.

**Test:** Release-readiness gates name stable and Rust 1.74 commands, and the
MSRV note now requires the exact toolchain or recorded CI evidence.

**Evidence:** `docs/release-readiness.md`, `Cargo.toml`, `Cargo.lock`, and
`tests/fixtures/scenario.v1.json`.
