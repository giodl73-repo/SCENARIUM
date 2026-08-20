# SCENARIUM

Deterministic scenario runs, inertia-first comparisons, and portable evidence
packets for Rust systems.

SCENARIUM extracts the product-neutral evidence spine already proven by RALLY
and the decision discipline used by SIGNALS:

```text
SCENARIO -> SEEDED RUNS -> INERTIA COMPARISON -> FINDINGS -> EVIDENCE PACKET
```

RALLY continues to own game and playtest mechanics. SIGNALS continues to own
feature-decision workflows, techniques, and review policy. SCENARIUM owns only
the reusable typed contracts between a scenario runner and its evidence.

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
    RunRecord::new(&scenario, RunVariant::Candidate("retry-v1".into()), "fixture")?;
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

## Non-goals

- No game, business-domain, or feature-scoring policy.
- No LLM or agent runtime.
- No solver, optimizer, statistical test, or visualization framework.
- No automatic recommendation that hides the underlying metric deltas.
- No mutable global run registry.

## License

[MIT](LICENSE) - Copyright (c) 2026 Gio Della-Libera.

