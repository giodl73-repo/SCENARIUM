# SCENARIUM Product Plan

## Product thesis

Working portfolio systems already produce deterministic simulations and
decision artifacts, but each consumer recreates run identity, baseline
comparison, findings, provenance, and evidence packaging. SCENARIUM provides
that shared contract without absorbing consumer semantics.

## Deletion target

Delete duplicated neutral run, metric, comparison, finding, and packet types
from adopters after compatibility tests prove that SCENARIUM preserves their
current behavior.

## Waves

### 1. Foundation

Ship one small crate with deterministic runs, inertia-first comparisons,
structured failures, provenance, evidence packets, and tests.

### 2. RALLY extraction

Add compatibility fixtures and migrate RALLY's neutral simulation/evidence
types. RALLY retains dice, turn order, board state, hidden information, and all
game-specific mechanics.

### 3. SIGNALS evidence bridge

Define an artifact adapter for SIGNALS JSON sidecars and append-only provenance.
SIGNALS retains techniques, skills, scoring rubrics, and decision campaigns.

### 4. Portfolio adoption

Pilot one domain simulation and one infrastructure benchmark. Add adapters only
when two consumers prove the same need.

## Disproof conditions

Stop or narrow the project if:

1. RALLY and a non-game consumer cannot share the same run/comparison contract;
2. adoption requires consumer policy inside SCENARIUM;
3. serde records are insufficient without a runtime or orchestration framework;
4. the extraction deletes less code than it introduces across the first two consumers.

