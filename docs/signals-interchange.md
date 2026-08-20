# SIGNALS artifact interchange

SCENARIUM retains a copy of the real SIGNALS sidecar introduced in SIGNALS
commit `ea5f090`. The source artifact is the competitive brief at
`experiments/S2-04-floor-variation/results/floor-20-run3-2026-03-15.md`.

The adapter in `tests/signals_interchange.rs` maps only neutral evidence:

- artifact identity and path;
- skill, topic, item, date, and skill version;
- input provenance;
- extension fields that SCENARIUM does not interpret.

Unknown fields remain in an explicit extension map and round-trip unchanged.
SIGNALS skills, techniques, rubrics, campaign sequencing, personas, and
recommendation policy are not dependencies and are not represented by
SCENARIUM.
