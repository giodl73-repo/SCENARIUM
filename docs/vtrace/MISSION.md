# Mission

## Scope

Repo: SCENARIUM

VTRACE adoption scope: implementation goals, public-contract hardening,
consumer compatibility, ecosystem readiness, and evidence for adoption.

## Mission Need

Rust products repeatedly implement deterministic run identity, baseline
comparison, findings, provenance, and evidence packaging. Those local copies
drift, hide comparison assumptions, and make cross-domain evidence difficult to
reuse. SCENARIUM must provide a small trustworthy contract that working owner
systems can adopt without surrendering their mechanics, metrics, workflow, or
decision policy.

## Users

| User | Need | Success Signal |
|---|---|---|
| Rust library maintainer | Add deterministic scenario evidence without building a framework. | First useful run/comparison packet requires only the crate and consumer-owned metrics. |
| RALLY maintainer | Remove neutral evidence plumbing while retaining game mechanics. | Compatibility fixtures pass and duplicated neutral types are deleted. |
| SIGNALS maintainer | Exchange append-only decision artifacts as typed evidence. | One JSON sidecar maps losslessly without importing SIGNALS methodology. |
| Domain simulation owner | Compare inertia/baselines with candidates reproducibly. | A non-game adopter produces the same result across repeated runs. |
| Evidence reviewer | Inspect negative, mixed, and provenance-sensitive outcomes. | Regressions, omitted metrics, invalid provenance, and replacement attempts are explicit failures or findings. |

## Operating Context

SCENARIUM is an embedded Rust crate used by test harnesses, simulations,
benchmarks, agent evaluations, and decision-support tools. Consumers execute
their own scenarios and supply their own metrics. SCENARIUM records, compares,
validates, and serializes the resulting evidence.

## Constraints

- Keep one public crate until independent release pressure proves a split.
- Keep consumer semantics and orchestration outside the crate.
- Preserve deterministic behavior for identical inputs.
- Treat serialized records as public compatibility surfaces.
- Prefer structured errors over silent defaults or dropped evidence.
- Require two real adopters before adding generalized adapter machinery.

## Non-Goals

- Agent or LLM runtime.
- Game, product, optimization, or statistical policy.
- Dashboard, registry, database, scheduler, or workflow engine.
- Automatic recommendation or confidence claims unsupported by consumer data.
- VTRACE concepts in the public API.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| RALLY and one non-game consumer share the core contract. | VAL-001 and VAL-003 | `VALIDATION.md` |
| SIGNALS sidecar provenance maps without semantic loss. | VAL-002 | `VALIDATION.md` |
| Adoption deletes more neutral code than it adds across two consumers. | repository diff analysis | EVID-ADOPT-001 |
| Invalid, selective, or replacement-shaped evidence cannot look successful. | adversarial tests and role review | EVID-ADV-001 |
| Public records have an explicit compatibility policy. | API/schema inspection | EVID-API-001 |

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `context/waves/2026-08-19-foundation/WAVE.md`
- RALLY `README.md` and `src/lib.rs`
- SIGNALS `README.md` and `PRINCIPLES.md`

