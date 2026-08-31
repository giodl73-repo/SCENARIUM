# SCENARIUM Pitfalls

## SCEN-PF-01: Neutral Contract Becomes Consumer Policy

**Status:** OPEN

**Pattern:** A repeated adopter request adds game mechanics, decision methods,
domain metrics, orchestration, recommendation policy, or visualization
behavior to the shared SCENARIUM crate.

**Actor:** Shared-crate maintainer, adopter maintainer, Runtime Boundary
Engineer, Consumer Advocate, or future agent responding to repeated adopter
requests.

**Task:** Add or review a shared API, adapter helper, descriptor, migration
path, or dependency adoption plan.

**Surface:** Public Rust API, README examples, RALLY/SIGNALS/CERES/TERRAIN/
BANISH adoption notes, VTRACE validation, and portfolio dependency records.

**Likely mistake:** Move consumer-owned mechanics, metrics, methods,
orchestration, recommendation, visualization, or release interpretation into
SCENARIUM because several adopters ask for adjacent convenience.

**Consequence:** SCENARIUM stops being a neutral evidence spine and starts
owning product policy that belongs in consumer repos.

**Owner:** SCENARIUM owns run/comparison/finding/provenance/packet contracts;
consumer repos and the Consumer Advocate own semantics, migration approval, and
deletion-ledger proof.

**Domain:** RALLY, SIGNALS, CERES, TERRAIN, BANISH, future RUNE descriptor
work, and portfolio dependency adoption.

**Detection difficulty:** Compatibility pressure often arrives as a small
convenience method, but repeated conveniences can quietly move consumer
ownership into the neutral crate.

**Structural solution:** Require Runtime Boundary Engineer and Consumer
Advocate review plus a named deletion ledger before adding adopter-shaped API.

**Evidence:** `CLAUDE.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
`docs/vtrace/VALIDATION.md`, and `docs/vtrace/EVIDENCE.md`.

**Test:** `tests/pitfall_policy.rs`

## SCEN-PF-02: Comparison Status Becomes Recommendation

**Status:** OPEN

**Pattern:** `Improved`, `Regressed`, `Mixed`, or `Equivalent` comparison
status is presented as a product decision, adoption recommendation, or
policy-ready result.

**Actor:** Report author, customer-demo author, adopter maintainer, Decision
Skeptic, public-doc author, or future agent summarizing comparison output.

**Task:** Interpret comparison status for README examples, evidence packets,
downstream reports, customer demos, research papers, or migration decisions.

**Surface:** `ComparisonStatus`, README examples, evidence packets, downstream
reports, customer demos, and future paper language.

**Likely mistake:** Treat metric-delta status as an adoption recommendation,
product decision, or policy-ready result without consumer-owned review.

**Consequence:** SCENARIUM can appear to decide what a product should do when it
only reports explicit comparison outcomes.

**Owner:** Decision Skeptic owns recommendation-boundary review; consumer repos
own decisions and public claims based on comparison evidence.

**Domain:** README examples, evidence packets, downstream reports, customer
demos, and future paper language.

**Detection difficulty:** Comparison statuses are concise and useful, so
downstream copy can turn metric deltas into decision authority without adding
consumer review.

**Structural solution:** Keep Decision Skeptic review and non-goal language
attached to public examples and reject automatic recommendation helpers.

**Evidence:** `README.md`, `.roles/parliament/decision-skeptic.md`,
`src/compare.rs`, and `tests/contract.rs`.

**Test:** `tests/pitfall_policy.rs`

## SCEN-PF-03: Packet Completeness Becomes Evidence Truthfulness

**Status:** OPEN

**Pattern:** A reference-closed, append-only packet is treated as proving that
the input metrics, artifacts, or findings are true, complete, representative,
or sufficient for external claims.

**Actor:** Evidence-packet author, downstream decision-system author,
research-paper author, release-note author, Adversarial Evidence Reviewer, or
future agent citing a closed packet.

**Task:** Build, publish, cite, or review an evidence packet, provenance report,
release note, research claim, or downstream decision record.

**Surface:** `EvidencePacket`, provenance records, VTRACE evidence/review docs,
release notes, research papers, and downstream decision systems.

**Likely mistake:** Read append-only reference closure as proof that
caller-supplied metrics, artifacts, or findings are true, complete,
representative, or decision-sufficient.

**Consequence:** Structurally valid evidence can become an overclaim about
truthfulness or sufficiency of inputs SCENARIUM did not validate.

**Owner:** SCENARIUM owns packet lineage and structure; Adversarial Evidence
Reviewer and callers own evidence truthfulness, completeness, and external
claim sufficiency.

**Domain:** Evidence packets, provenance reports, release notes, research
papers, and downstream decision systems.

**Detection difficulty:** Packet closure is a strong structural guarantee, so
it can be overread as epistemic validation of caller-supplied evidence.

**Structural solution:** Preserve adversarial evidence review and wording that
packets prove lineage and structure, not truth or completeness.

**Evidence:** `src/evidence.rs`, `docs/vtrace/REVIEW.md`,
`docs/vtrace/EVIDENCE.md`, and
`.roles/stakeholders/adversarial-evidence-reviewer.md`.

**Test:** `tests/pitfall_policy.rs`

## SCEN-PF-04: Compatibility Proof Becomes Migration Approval

**Status:** MITIGATED

**Pattern:** Passing RALLY, SIGNALS, CERES, TERRAIN, or BANISH compatibility
evidence is treated as approval to migrate consumers, expand SCENARIUM, publish
to a registry, or add accessors despite stop-gate results.

**Actor:** Migration owner, adopter maintainer, portfolio dependency reviewer,
release-readiness reviewer, registry publisher, or future agent reading
compatibility evidence.

**Task:** Decide whether compatibility evidence approves consumer migration,
SCENARIUM expansion, registry publication, new accessors, or portfolio snapshot
adoption.

**Surface:** Adoption plans, VTRACE evidence, release-readiness docs, consumer
PRs, dependency-system records, and portfolio snapshots.

**Likely mistake:** Treat green compatibility tests as migration or publication
approval while ignoring deletion-ledger, stop-gate, and consumer-owned review
results.

**Consequence:** Consumers can inherit unnecessary abstraction cost, registry
claims can overstate readiness, and SCENARIUM can expand around failed
migration economics.

**Owner:** Consumer Advocate owns migration economics and deletion ledgers;
SCENARIUM owns neutral compatibility evidence; portfolio policy owns registry
publication gates.

**Domain:** Adoption plans, dependency-system records, release readiness,
consumer PRs, and portfolio snapshots.

**Detection difficulty:** VTRACE records many verified adoption proofs, but
TERRAIN and BANISH deliberately prove stop gates as well as compatibility.

**Structural solution:** Keep adoption approval tied to consumer-owned deletion
ledgers, explicit stop-gate outcomes, and portfolio policy against registry
publication. `docs/adoption-gates.v1.json` now records per-consumer approval
state so compatibility evidence cannot be promoted into migration,
publication, accessor-expansion, or consumer-policy approval.

**Evidence:** `docs/vtrace/EVIDENCE.md`, `docs/vtrace/VALIDATION.md`,
`README.md`, and `docs/release-readiness.md`.

**Test:** `tests/pitfall_policy.rs` parses `docs/adoption-gates.v1.json` and
requires compatibility to remain separate from migration approval, registry
publication, and SCENARIUM expansion.

## SCEN-PF-05: MSRV Claim Uses Ambient Stable Toolchain

**Status:** MITIGATED

**Pattern:** Stable `cargo test` or CI-memory is used as current Rust 1.74 MSRV
evidence even when the exact 1.74 toolchain is not installed or the documented
MSRV command has not run locally.

**Domain:** Release readiness, package inspection, portfolio scoring, and
customer dependency claims.

**Detection difficulty:** The stable Rust gates pass cleanly, and the release
doc already records prior CI evidence, so local agents can miss that they have
not reproduced the exact MSRV lane.

**Structural solution:** State that Rust 1.74 evidence requires the exact
toolchain or recorded CI evidence; do not treat stable-toolchain success as a
substitute.

**Evidence:** `docs/release-readiness.md`,
`C:\Users\giodl\.cargo\bin\rustup.exe run 1.74.0 cargo test --locked`, and
`cargo test --locked`.
