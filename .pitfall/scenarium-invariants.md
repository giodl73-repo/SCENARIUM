# SCENARIUM Invariants

## SCEN-I-01: Run Identity Includes Seed And Variant Kind

**Status:** MITIGATED

**Claim:** Run identities distinguish seed labels and baseline/candidate
variant kinds so same-named variants cannot collide across comparison roles.

**Why it matters:** A collision can make a comparison appear reproducible while
silently comparing the wrong evidence role.

**Enforcement:** Contract tests assert distinct run IDs for seed changes and
same-id baseline/candidate variants.

**Evidence:** `tests/contract.rs`, `src/model.rs`, and
`cargo test run_identity_includes_seed_and_variant_kind`.

## SCEN-I-02: Comparisons Reject Empty, Wrong-Variant, And Non-Finite Results

**Status:** MITIGATED

**Claim:** Comparison APIs fail structurally for empty metric sets, baseline-as-
candidate comparisons, and non-finite derived deltas.

**Why it matters:** Empty or invalid comparisons are the easiest path from
evidence plumbing to plausible but meaningless result summaries.

**Enforcement:** Contract tests cover empty metric sets, wrong variant roles,
and overflow-derived non-finite metrics.

**Evidence:** `tests/contract.rs`, `src/compare.rs`, and
`cargo test comparison_rejects_empty_wrong_variant_and_overflow`.

## SCEN-I-03: Schema Fixtures Remain Canonical

**Status:** MITIGATED

**Claim:** Persisted JSON records use `scenarium.v1`, reject unsupported
schemas, and retain canonical fixture readability across compatible releases.

**Why it matters:** Consumers need stable evidence documents that fail loudly
when schema meaning changes.

**Enforcement:** Versioned decoding tests, retained fixtures, and release
readiness policy protect schema compatibility.

**Evidence:** `src/document.rs`, `tests/fixtures/scenario.v1.json`,
`tests/contract.rs`, and `docs/release-readiness.md`.

## SCEN-I-04: Evidence Packets Are Append-Only And Reference-Closed

**Status:** MITIGATED

**Claim:** Packets cannot replace artifacts under an existing name, and
comparisons cannot be included unless both referenced runs are already present.

**Why it matters:** Evidence packets are useful only if packet contents cannot
quietly detach comparisons from their runs or overwrite artifacts.

**Enforcement:** Packet tests reject missing runs, duplicate artifacts, and
non-canonical packet construction.

**Evidence:** `src/evidence.rs`, `tests/contract.rs`, and
`cargo test packet_is_canonical_append_only_and_reference_closed`.

## SCEN-I-05: SIGNALS Interchange Keeps Unknown Fields Visible

**Status:** MITIGATED

**Claim:** SCENARIUM maps neutral SIGNALS sidecar provenance while unknown
fields remain observable and SIGNALS methodology stays outside the crate.

**Why it matters:** Interchange would be unsafe if SCENARIUM silently dropped
caller context or imported SIGNALS decision practice.

**Enforcement:** SIGNALS interchange tests round-trip the retained fixture,
preserve unknown fields, and reject unsupported schemas.

**Evidence:** `tests/signals_interchange.rs`, `docs/signals-interchange.md`,
and `cargo test --test signals_interchange`.

## SCEN-I-06: Neutrality, Recommendation, And Packet Truth Boundaries Are Machine-Readable

**Status:** VERIFIED

**Claim:** SCENARIUM keeps neutral-contract, comparison-recommendation, and
evidence-truth boundaries in a machine-readable manifest that is also routed
through roles, README non-goals, VTRACE evidence, and focused tests.

**Why it matters:** A useful shared evidence crate fails if adopter convenience
imports consumer policy, metric status becomes a recommendation, or structural
packet closure is mistaken for truthfulness.

**Enforcement:** `tests/pitfall_policy.rs` asserts the manifest, role routing,
README boundaries, VTRACE evidence wording, contract tests, comparison states,
and evidence-packet failure modes.

**Evidence:** `docs/pitfall-boundaries.v1.json`, `.roles/ROLE.md`,
`README.md`, `docs/vtrace/EVIDENCE.md`, and `tests/pitfall_policy.rs`.
