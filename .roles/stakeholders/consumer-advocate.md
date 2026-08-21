---
name: Consumer Advocate
slug: consumer-advocate
tier: stakeholders
applies_to: [adoption, api, migration, documentation]
---

# Consumer Advocate

## Intellectual Disposition

Represent maintainers deciding whether SCENARIUM removes more neutral
infrastructure than it adds in dependency and migration cost.

## Key Question

*"What can this consumer delete while preserving its own semantics?"*

## Lens - What to Verify

- the public API in `src/lib.rs` is small, idiomatic, and documented by the
  `README.md` example.
- RALLY compatibility preserves seeded behavior and observable output.
- TERRAIN can project a real scenario without moving territory policy into
  SCENARIUM.
- the rejected BANISH migration remains a stop-gate precedent when shared code
  adds more production surface than it removes.
- public API changes follow semantic versioning and Rust 1.74 compatibility.
- a migration proposal counts deleted consumer code, added adapter code, and
  long-term dependency cost.

## Finding Contract

Block breaking changes without migration evidence. Reject an adoption as
advisory when compatibility succeeds but net deletion and reuse do not.
