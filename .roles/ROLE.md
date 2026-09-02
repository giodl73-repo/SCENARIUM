# SCENARIUM Review Panel

Use this panel when changing SCENARIUM's neutral scenario, comparison, or
evidence contracts. Consumer-specific execution and decision policy remain in
consumer repositories.

## Active Roles

| Role | Protects | Invoke when |
|---|---|---|
| [Runtime Boundary Engineer](parliament/runtime-boundary-engineer.md) | Product-neutral ownership boundary | Adding capabilities, adapters, dependencies, or policy |
| [Simulation Auditor](parliament/simulation-auditor.md) | Determinism and replay | Changing seeds, comparison, ordering, or serialization |
| [Decision Skeptic](parliament/decision-skeptic.md) | Honest inertia and negative results | Changing statuses, findings, summaries, or recommendations |
| [Evidence Custodian](parliament/evidence-custodian.md) | Provenance and append-only packets | Changing evidence packets, artifacts, digests, or document envelopes |
| [Consumer Advocate](stakeholders/consumer-advocate.md) | Reusable contracts without migration tax | Changing public types or proposing a consumer migration |
| [Adversarial Evidence Reviewer](stakeholders/adversarial-evidence-reviewer.md) | Resistance to trustworthy-looking bad evidence | Changing validation, caller-supplied data, or trust claims |

## Core Tensions

| Pulls | Against | Because |
|---|---|---|
| Runtime Boundary Engineer | Consumer Advocate | A convenient consumer feature can import product policy into the neutral crate. |
| Simulation Auditor | Consumer Advocate | Stronger determinism constraints can make legitimate adapters harder to implement. |
| Decision Skeptic | Evidence Custodian | A complete packet can still frame selective or misleading comparisons. |
| Evidence Custodian | Adversarial Evidence Reviewer | Preserved provenance proves lineage, not truthfulness or completeness. |
| Consumer Advocate | Decision Skeptic | A simpler result may hide mixed outcomes that consumers need to interpret themselves. |

## Review Order

1. Simulation Auditor establishes deterministic behavior.
2. Evidence Custodian and Adversarial Evidence Reviewer establish what the
   packet proves and what it cannot prove.
3. Runtime Boundary Engineer rejects consumer policy in shared contracts.
4. Decision Skeptic checks that regressions and inertia remain visible.
5. Consumer Advocate evaluates whether the surviving contract earns its
   migration and maintenance cost.

Correctness, provenance loss, silent replacement, and hidden decision policy
are blocking. Adoption convenience is advisory unless a named consumer can
delete more duplicated neutral code than the shared dependency adds.

## PITFALL gate routing

Invoke the Runtime Boundary Engineer and Consumer Advocate before repeated
adopter requests add game mechanics, decision methods, domain metrics,
orchestration policy, recommendation policy, visualization behavior, or
consumer semantics to the shared crate.

Invoke the Decision Skeptic before `Improved`, `Regressed`, `Mixed`, or
`Equivalent` comparison status is used as an automatic recommendation, product
decision, adoption recommendation, policy-ready result, customer decision
authority, or migration approval.

Invoke the Evidence Custodian and Adversarial Evidence Reviewer before a
reference-closed packet is used to claim that caller-supplied metrics,
artifacts, or findings are true, complete, representative, or sufficient for
external claims.
