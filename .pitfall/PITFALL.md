# SCENARIUM PITFALL Index

SCENARIUM uses PITFALL to preserve doctrine for deterministic scenario
contracts, inertia-first comparison, append-only evidence packets, provenance,
consumer-boundary discipline, and release-evidence gates.

| Namespace | Kind | Path | Owner |
|---|---|---|---|
| `scenarium` | `principles` | [scenarium-principles.md](scenarium-principles.md) | SCENARIUM maintainer |
| `scenarium` | `invariants` | [scenarium-invariants.md](scenarium-invariants.md) | SCENARIUM maintainer |
| `scenarium` | `pitfalls` | [scenarium-pitfalls.md](scenarium-pitfalls.md) | SCENARIUM maintainer |

## Integration

- ROLES: `.roles/ROLE.md` covers deterministic replay, evidence custody,
  decision skepticism, product-neutral ownership, consumer migration cost, and
  adversarial evidence review.
- VTRACE: `docs/vtrace/TRACE.md`, `VALIDATION.md`, `EVIDENCE.md`, and
  `REVIEW.md` provide the canonical requirement, validation, and evidence
  pointers for RALLY, SIGNALS, CERES, TERRAIN, BANISH, and release-readiness
  gates.
- Tests: Rust contract tests, SIGNALS interchange tests, clippy, package/docs
  inspection, release-readiness evidence, and PITFALL validators are the
  evidence hooks.
