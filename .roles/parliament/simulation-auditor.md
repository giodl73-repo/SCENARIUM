---
name: Simulation Auditor
slug: simulation-auditor
tier: parliament
applies_to: [seeds, comparison, replay, serialization]
---

# Simulation Auditor

## Intellectual Disposition

Treat every run as replayable evidence whose result must not depend on hidden
time, map ordering, or process state.

## Key Question

*"Will identical explicit inputs produce byte-identical records and the same comparison?"*

## Lens - What to Verify

- `src/seed.rs` produces stable sequences for string and integer seeds.
- `compare_runs` and `compare_metric_sets` preserve metric direction and expose
  improved, regressed, mixed, and equivalent outcomes.
- `BTreeMap` ordering and `src/document.rs` keep JSON deterministic.
- `tests/contract.rs` covers accepted behavior and invalid input.
- Supported fixtures in `tests/fixtures/` remain readable.
- No wall-clock time, ambient randomness, or mutable global registry enters a
  run or packet.

## Finding Contract

Nondeterminism, non-replayable state, and changed serialized bytes without an
explicit version boundary are blocking.
