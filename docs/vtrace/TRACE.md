# Trace Matrix

| Requirement ID | Parent Need | Specification Item | Code Rigor | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | Reproducible evidence | SPEC-001/NF-001 | CR-002/004 | WP-001 | seed/run modules and golden fixtures | deterministic tests | VAL-001/003 | EVID-001 | implemented/expand |
| REQ-002 | Visible inertia and identity | SPEC-001 | CR-003/007 | WP-001 | scenario/run types | API tests | VAL-001/003 | EVID-002 | implemented |
| REQ-003 | Complete trustworthy comparison | SPEC-002/NF-003 | CR-002/003/004 | WP-001 | comparison module | negative/table tests | VAL-001/003 | EVID-003 | implemented/expand |
| REQ-004 | Preserve outcome ambiguity | SPEC-002 | CR-002 | WP-001 | comparison statuses | table tests | reviewer scenario | EVID-004 | implemented |
| REQ-005 | Structured findings | SPEC-003 | CR-003/007 | WP-001 | findings/status module | serialization tests | evidence review | EVID-005 | partial |
| REQ-006 | Provenance and append-only packets | SPEC-004 | CR-003/004/007 | WP-001/WP-003 | packet/provenance modules | negative/round-trip tests | VAL-002 | EVID-006 | partial |
| REQ-007 | Versioned public records | SPEC-005/IF-002 | CR-004/007 | WP-001/WP-005 | document envelopes/fixtures | golden compatibility tests | release review | EVID-API-001 | proposed |
| REQ-008 | Safe RALLY extraction | SPEC-006/IF-003 | CR-002/004 | WP-002 | RALLY adapter/fixtures | dual-repo tests | VAL-001 | EVID-RALLY-001 | proposed |
| REQ-009 | SIGNALS interchange | SPEC-007/IF-004 | CR-003/007 | WP-003 | sidecar adapter/fixture | round-trip test | VAL-002 | EVID-SIGNALS-001 | proposed |
| REQ-010 | Non-game proof and deletion | SPEC-008 | CR-008 | WP-004 | adopter integration | integration/diff analysis | VAL-003 | EVID-ADOPT-001 | proposed |
| REQ-011 | Ecosystem maintenance contract | SPEC-005/IF-001 | CR-005/007/008 | WP-005 | Cargo/docs/fixtures | package/docs inspection | release review | EVID-RELEASE-001 | proposed |
| REQ-012 | Packet integrity | SPEC-004 | CR-003/004 | WP-001 | packet validation | adversarial tests | VAL-002/003 | EVID-ADV-001 | proposed |
| REQ-013 | Consistent record invariants | SPEC-009 | CR-003/004/009 | WP-001 | constructors, serde, validation | invalid fixture/mutation tests | VAL-004 | EVID-ADV-001 | proposed |
| REQ-014 | Collision-resistant run identity | SPEC-009 | CR-004 | WP-001 | seed and run identity | collision tests | VAL-001/003 | EVID-ADV-001 | proposed |
| REQ-015 | Valid non-empty finite comparison | SPEC-010 | CR-002/003/004/010 | WP-001 | comparison logic | boundary/negative tests | VAL-004 | EVID-ADV-001 | proposed |
| REQ-016 | Explicit bounded RNG guarantee | SPEC-010 | CR-002/004 | WP-001 | seed module | algorithm/distribution tests | VAL-003/004 | EVID-ADV-001 | proposed |
| REQ-D-001 | Deferred `no_std` support | deferred | n/a | deferred | none | named-adopter review | deferred scenario | n/a | deferred |
| REQ-D-002 | Deferred statistical inference | deferred | n/a | deferred | none | repeated-need review | deferred scenario | n/a | deferred |
| REQ-D-003 | Deferred runtime orchestration | deferred | n/a | deferred | none | separate-owner review | deferred scenario | n/a | deferred |
| REQ-001 | Deterministic serialization constraint | SPEC-NF-001 | CR-004/007 | WP-001 | crate and golden fixtures | repeated tests | VAL-001/003 | EVID-API-001 | proposed |
| REQ-011 | Dependency restraint constraint | SPEC-NF-002 | CR-008 | WP-005 | Cargo manifest | manifest review | VAL-005 | EVID-RELEASE-001 | accepted |
| REQ-003/006 | Failure visibility constraint | SPEC-NF-003 | CR-003/004 | WP-001 | comparison and packet validation | negative tests | VAL-004 | EVID-ADV-001 | accepted |
| REQ-007 | Compatibility constraint | SPEC-NF-004 | CR-007 | WP-001/WP-005 | retained schema fixtures | compatibility tests | VAL-005 | EVID-API-001 | proposed |
| discovery | First non-game adopter selection | SPEC-UNK-001 | n/a | WP-004 | adopter analysis | review | VAL-003 | EVID-ADOPT-001 | discovery |
| discovery | Adapter ownership placement | SPEC-UNK-002 | n/a | WP-003 | adapter analysis | review | VAL-002 | EVID-SIGNALS-001 | discovery |
| discovery | MSRV selection | SPEC-UNK-003 | n/a | WP-005 | dependency/toolchain analysis | review | VAL-005 | EVID-RELEASE-001 | discovery |
