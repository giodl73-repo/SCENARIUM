# Wave: Foundation

## Goal

Create a publishable Rust crate for deterministic scenario records,
inertia-first comparison, structured findings, provenance, and evidence packets.

## Thesis

RALLY and SIGNALS already prove the owner behavior. A small shared contract can
remove duplicated evidence plumbing without taking ownership of game mechanics,
decision methods, domain metrics, or orchestration.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | complete | Repo skeleton, docs, skills, crate, and tests. |
| 02 | Contract hardening | complete | Added schema versioning, invariant-safe decoding, strict comparison, canonical packets, golden fixtures, and internal modules. |
| 03 | RALLY compatibility | pending | Prove type and fixture compatibility before migration. |
| 04 | SIGNALS evidence bridge | pending | Map one append-only JSON sidecar into a packet. |
| 05 | First non-game consumer | pending | Prove reuse and measure deleted code. |

## Success criteria

- Inertia is represented as a typed baseline.
- Accepted, mixed, equivalent, and regressed comparisons remain observable.
- Invalid metrics and artifact replacement produce structured failures.
- Serialization is deterministic for identical inputs.
- RALLY and SIGNALS ownership boundaries are explicit.
