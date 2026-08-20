# Evidence Ledger

## Scope

SCENARIUM foundation, contract hardening, adoption, and release readiness.

## Evidence Records

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |
|---|---|---|---|---|---|
| EVID-001 | test | `cargo test tests::seed_is_repeatable` | Identical labels produce identical sequences. | Test passes. | passed |
| EVID-002 | test/inspection | `cargo test tests::inertia_comparison_reports_improvement` and `RunVariant::Inertia` inspection | Inertia is a typed baseline with stable run identity. | Test passes and type exists. | passed |
| EVID-003 | test | `cargo test tests::invalid_and_duplicate_metrics_are_structured_failures` | Invalid and duplicate metrics return typed errors. | Test passes. | passed |
| EVID-004 | test | `cargo test tests::mixed_outcomes_remain_visible` | Mixed outcomes remain explicit. | Test passes. | passed |
| EVID-005 | test | `cargo test findings_drive_structured_run_status` | Findings distinguish severity and determine run status. | Test passes. | passed |
| EVID-006 | test | `cargo test tests::packet_rejects_artifact_replacement tests::packet_json_is_deterministic` | Duplicate replacement fails and identical packet inputs serialize identically. | Tests pass. | passed |
| EVID-CI-001 | CI | GitHub Actions run `32330439679` | fmt, Clippy, and tests pass. | Workflow completed successfully. | passed |
| EVID-API-001 | fixture/review | `tests/fixtures/scenario.v1.json`; `cargo test retained_v1_fixture_round_trips_canonically`; README compatibility policy | Versioned schema and compatibility policy exist. | `scenarium.v1`, retained fixture, Rust 1.74, semver, feature, and dependency rules are present. | passed |
| EVID-ADV-001 | test/review | `cargo test --test contract`; `docs/vtrace/REVIEW.md` | Incomplete or misleading evidence is rejected or reported. | Ten contract tests cover identity, RNG, comparison states/failures, validated decoding, findings, digests, append-only behavior, and packet closure. | passed |
| EVID-RALLY-001 | integration | WP-002 RALLY compatibility fixture and dual-repo test commands | Accepted and failure behavior is equivalent; deletion plan exists. | Evidence collection deferred until WP-002 entry criteria pass. | deferred |
| EVID-SIGNALS-001 | fixture/review | WP-003 SIGNALS sidecar round-trip | Provenance survives and methodology remains outside SCENARIUM. | Evidence collection deferred until WP-003 entry criteria pass. | deferred |
| EVID-ADOPT-001 | integration/analysis | WP-004 adopter tests and before/after diff | Non-game adoption passes with net neutral-code deletion. | Evidence collection deferred until an adopter is selected. | deferred |
| EVID-RELEASE-001 | package/review | WP-005 `cargo package`, docs, MSRV, semver, and schema checks | Release contract is explicit and package-ready. | Evidence collection deferred until WP-004 L2 closure. | deferred |

## Claim discipline

- Foundation evidence supports a crate foundation, not ecosystem maturity.
- No RALLY migration claim is valid before EVID-RALLY-001.
- No generality claim is valid before EVID-ADOPT-001.
- No stable schema or crates.io readiness claim is valid before EVID-API-001
  and EVID-RELEASE-001.
