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
| EVID-RALLY-001 | integration | RALLY commit `6b0bbc6`; `cargo test --test scenarium_compat`; `cargo test` | Accepted and failure behavior is equivalent; stricter incompatibilities are explicit; game mechanics remain local; deletion plan exists. | Three compatibility tests and the full RALLY suite pass. The ledger names seven duplicate types and source ranges; empty and non-finite comparisons fail structurally in SCENARIUM. | passed |
| EVID-SIGNALS-001 | fixture/review | SIGNALS commit `ea5f090`; `cargo test --test signals_interchange`; `docs/signals-interchange.md` | Provenance survives, unknown fields remain visible, and methodology remains outside SCENARIUM. | Real skill/topic/item/date/version/input provenance and artifact identity round-trip; extension fields survive; unsupported schema fails structurally. | passed |
| EVID-ADOPT-001 | integration/analysis | CERES `71c7ef2`; `cargo test`; repeated real smithing comparison; `docs/scenarium-adoption.md` | Non-game adoption passes with deterministic comparison and net neutral-code deletion. | Repeated comparison and packet bytes match; packets use `scenarium.v1`; CERES removes its duplicate evidence family; production Rust is 122 added/123 removed. | passed |
| EVID-TERRAIN-001 | integration/falsification | TERRAIN `29af76f`; retained steady-state/risky-reassignment fixtures; repeated `packet-csv`; TERRAIN `docs/scenarium-adoption.md` | A second direct non-game adopter either deletes a repeated neutral seam or stops without expanding SCENARIUM. | Twelve packet artifacts are byte-identical across repeated runs and use `scenarium.v1`; all prior TERRAIN artifacts remain. No duplicate neutral type family exists, so the deletion gate fails and broader abstraction stops. | passed |
| EVID-BANISH-001 | integration/falsification | BANISH decision `43f01b7`; rejected prototype `4363c65`; SCENARIUM accessor proof `197e503`; BANISH `docs/scenarium-adoption.md` | A RALLY consumer either replaces duplicate neutral evidence with net simplification or stops without changing SCENARIUM. | First Winter deterministic tests pass and single-run, variant, and profile CLI output hashes match the baseline. The production-only diff is +362/-118, so the prototype and dependency are rejected and the accessor branch remains unmerged. | passed |
| EVID-RELEASE-001 | package/review | `cargo package --locked`; `cargo doc --locked --no-deps`; `rustup run 1.74.0 cargo test --locked`; `docs/release-readiness.md` | Release contract is explicit and package-ready. | Stable and Rust 1.74 gates pass; lock format is Cargo 1.74-compatible; metadata, policies, and retained fixtures are packaged. | passed |

## Claim discipline

- Foundation evidence supports a crate foundation, not ecosystem maturity.
- RALLY compatibility is proven; neutral-type deletion remains consumer-gated by
  the migration ledger.
- Generality is supported by RALLY, SIGNALS, and CERES. TERRAIN proves the
  existing contracts can project another domain but finds no neutral deletion.
  BANISH proves compatibility with a game consumer but finds adapter cost
  exceeds deletion. Both stop gates block broader abstraction claims, accessor
  additions, and RUNE descriptor work.
- Package readiness is supported by EVID-API-001 and EVID-RELEASE-001.
  Registry publication is prohibited by portfolio policy.
