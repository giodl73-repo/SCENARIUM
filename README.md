# SCENARIUM

Deterministic scenario runs, inertia-first comparisons, and portable evidence
packets for Rust systems.

## Compare runs in your browser

Open the [Rust/WASM comparison workbench](https://giodl73-repo.github.io/SCENARIUM/)
to load two `scenarium.v1` run records, inspect metric deltas, findings and
caller-supplied provenance, and download the native comparison. Six synthetic
examples cover all four outcomes, target directions, and improved metrics with
error findings still present. Your inputs stay in the tab.

The page exposes the existing Rust decoder and comparison core. It does not
execute experiments or recommend adoption. Unknown fields are reported;
original input downloads retain uploaded UTF-8 text until editing or replacement.
Source, compiled WASM, dependency notices and build provenance are downloadable
from the page. See [BUILD.md](BUILD.md) for reproduction and limits, and
[browser verification scope](docs/vtrace/BROWSER_COMPARISON.md).

This unpublished first-party companion has its own `web/adapter` workspace and
toolchain. The foundation's Rust 1.74 support, API and dependency policy remain
unchanged. No external adoption or neutral-code deletion is claimed by the UI.

SCENARIUM extracts the product-neutral evidence spine already proven by RALLY
and the decision discipline used by SIGNALS:

```text
SCENARIO -> SEEDED RUNS -> INERTIA COMPARISON -> FINDINGS -> EVIDENCE PACKET
```

RALLY continues to own game and playtest mechanics. SIGNALS continues to own
feature-decision workflows, techniques, and review policy. SCENARIUM owns only
the reusable typed contracts between a scenario runner and its evidence.
Repeated adopter requests do not move game mechanics, decision methods, domain
metrics, orchestration, recommendation policy, or visualization behavior into
the shared crate without a named deletion ledger and consumer-owned review.

The retained [SIGNALS interchange proof](docs/signals-interchange.md) maps real
artifact provenance without importing SIGNALS methodology.

## First command

```powershell
cargo test
```

## Example

```rust
use scenarium::{
    compare_runs, Metric, MetricDirection, RunRecord, RunVariant, Scenario,
};

let scenario = Scenario::new("checkout", "Checkout reliability", "seed-1")?;

let mut inertia = RunRecord::new(&scenario, RunVariant::Inertia, "fixture")?;
inertia.record_metric(Metric::new(
    "completion_rate",
    0.61,
    MetricDirection::HigherIsBetter,
)?)?;

let mut candidate =
    RunRecord::new(&scenario, RunVariant::candidate("retry-v1")?, "fixture")?;
candidate.record_metric(Metric::new(
    "completion_rate",
    0.83,
    MetricDirection::HigherIsBetter,
)?)?;

let comparison = compare_runs(&inertia, &candidate)?;
assert_eq!(comparison.status().as_str(), "improved");
# Ok::<(), scenarium::Error>(())
```

## Foundation contracts

- deterministic string- and integer-seeded pseudo-random generation;
- explicit inertia and named-baseline run variants;
- typed metric direction rather than stringly-typed comparison policy;
- structured findings and run validation status;
- comparison reports with improved, regressed, mixed, and equivalent outcomes;
- provenance records supplied by the caller rather than hidden wall-clock state;
- evidence packets that reject artifact replacement and serialize deterministically.

## Design inheritance

From RALLY:

- seeded execution;
- run, metric, finding, comparison, and packet shapes;
- consumer policy remains outside the shared crate.

From SIGNALS:

- inertia is a first-class competitor;
- artifacts carry provenance;
- negative and falsifying results are valuable evidence;
- independent runs remain self-contained and can be synthesized intentionally.

TERRAIN provides a second direct non-game pressure test. It consumes the
existing run, comparison, finding, provenance, and packet contracts for one
real territory-plan scenario while retaining every territory-specific report.
The audit found no duplicate neutral type family to delete, so the result is a
bounded additive projection and an explicit stop gate rather than a reason to
expand SCENARIUM.

BANISH provides a game-consumer migration pressure test. Its retained First
Winter prototype preserves deterministic behavior and byte-identical CLI output,
but adds 362 production Rust lines while removing 118. The migration and
SCENARIUM accessor branch are therefore rejected: compatibility alone is not
enough to justify abstraction.

## Implementation governance

The controlled implementation goals, requirements, work packages, verification
plan, and adoption gates live in [`docs/vtrace/`](docs/vtrace/README.md).
The package-readiness evidence is summarized in
[`docs/release-readiness.md`](docs/release-readiness.md). Registry publication
is prohibited by portfolio policy; SCENARIUM remains source-consumed.

## Compatibility policy

- The minimum supported Rust version is **1.74**.
- Rust API changes follow semantic versioning.
- Persisted JSON uses an explicit `scenarium.v1` document envelope.
- Supported schema fixtures remain readable across compatible crate releases.
- Schema field removals, renames, enum reinterpretations, and semantic changes
  require a new schema version and a documented migration path.
- The foundation has no optional Cargo features. New dependencies or features
  require a named capability and adopter; consumer-specific adapters remain in
  consumer repositories until repeated use proves a shared boundary.

## Non-goals

- No game, business-domain, or feature-scoring policy.
- No LLM or agent runtime.
- No solver, optimizer, statistical test, or visualization framework.
- No automatic recommendation that hides the underlying metric deltas.
- No product decision, adoption recommendation, policy-ready result, customer
  decision authority, or migration approval from comparison status alone.
- No truthfulness, completeness, representativeness, or external-claim
  sufficiency guarantee from packet closure alone.
- No mutable global run registry.

## License

[MIT](LICENSE) - Copyright (c) 2026 Gio Della-Libera.
