fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn neutral_contract_does_not_absorb_consumer_policy() {
    // Checks SCEN-PF-01: neutral contracts must not become consumer policy.
    let readme = include_str!("../README.md");
    let plan = include_str!("../PRODUCT_PLAN.md");
    let roles = include_str!("../.roles/ROLE.md");
    let validation = include_str!("../docs/vtrace/VALIDATION.md");
    let boundary_manifest = include_str!("../docs/pitfall-boundaries.v1.json");

    assert!(normalized(readme).contains("SCENARIUM owns only the reusable typed contracts"));
    assert!(
        normalized(readme).contains("consumer-specific adapters remain in consumer repositories")
    );
    assert!(normalized(readme).contains("named deletion ledger and consumer-owned review"));
    assert!(plan.contains("SIGNALS retains techniques, skills, scoring rubrics"));
    assert!(roles.contains("Runtime Boundary Engineer"));
    assert!(roles.contains("Consumer Advocate"));
    assert!(roles.contains("consumer semantics to the shared crate"));
    assert!(validation.contains("consumer metrics stay local"));
    assert!(boundary_manifest.contains("SCEN-PF-01"));
    assert!(boundary_manifest.contains("consumer semantics in shared crate"));
    assert!(boundary_manifest.contains("neutral duplicate deletion proof"));
}

#[test]
fn comparison_status_does_not_become_recommendation() {
    // Checks SCEN-PF-02: comparison status is not decision authority.
    let readme = include_str!("../README.md");
    let decision_skeptic = include_str!("../.roles/parliament/decision-skeptic.md");
    let contract_tests = include_str!("contract.rs");
    let compare_src = include_str!("../src/compare.rs");
    let roles = include_str!("../.roles/ROLE.md");
    let boundary_manifest = include_str!("../docs/pitfall-boundaries.v1.json");

    assert!(readme.contains("No automatic recommendation"));
    assert!(readme.contains("No product decision"));
    assert!(decision_skeptic.contains("Protect the evidence needed to decide not to adopt"));
    assert!(decision_skeptic.contains("Block APIs that hide negative evidence"));
    assert!(normalized(roles).contains("customer decision authority"));
    assert!(contract_tests.contains("ComparisonStatus::Regressed"));
    assert!(contract_tests.contains("ComparisonStatus::Mixed"));
    assert!(contract_tests.contains("ComparisonStatus::Equivalent"));
    assert!(compare_src.contains("ComparisonStatus"));
    assert!(boundary_manifest.contains("SCEN-PF-02"));
    assert!(boundary_manifest.contains("automatic recommendation"));
    assert!(boundary_manifest.contains("migration approval"));
}

#[test]
fn packet_closure_does_not_prove_evidence_truthfulness() {
    // Checks SCEN-PF-03: packets preserve lineage, not truth.
    let evidence_src = include_str!("../src/evidence.rs");
    let evidence = include_str!("../docs/vtrace/EVIDENCE.md");
    let requirements = include_str!("../docs/vtrace/REQUIREMENTS.md");
    let reviewer = include_str!("../.roles/stakeholders/adversarial-evidence-reviewer.md");
    let pitfalls = include_str!("../.pitfall/scenarium-pitfalls.md");
    let readme = include_str!("../README.md");
    let roles = include_str!("../.roles/ROLE.md");
    let boundary_manifest = include_str!("../docs/pitfall-boundaries.v1.json");

    assert!(evidence_src.contains("EvidencePacket"));
    assert!(evidence_src.contains("MissingPacketRun"));
    assert!(evidence_src.contains("DuplicateArtifact"));
    assert!(evidence.contains("Foundation evidence supports a crate foundation"));
    assert!(requirements.contains("caller-supplied provenance"));
    assert!(reviewer.contains("trustworthy-looking packet"));
    assert!(reviewer.contains("SCENARIUM-verified facts"));
    assert!(pitfalls.contains("lineage and structure, not truth"));
    assert!(readme.contains("No truthfulness"));
    assert!(roles.contains("representative, or sufficient"));
    assert!(boundary_manifest.contains("SCEN-PF-03"));
    assert!(boundary_manifest.contains("packet closure proves truthfulness"));
    assert!(boundary_manifest.contains("lineage and structure boundary"));
}

#[test]
fn compatibility_proof_does_not_become_migration_approval() {
    // Checks SCEN-PF-04: compatibility evidence is not migration approval.
    let readme = include_str!("../README.md");
    let release = include_str!("../docs/release-readiness.md");
    let evidence = include_str!("../docs/vtrace/EVIDENCE.md");
    let consumer = include_str!("../.roles/stakeholders/consumer-advocate.md");
    let gates: serde_json::Value =
        serde_json::from_str(include_str!("../docs/adoption-gates.v1.json"))
            .expect("adoption gates must remain valid JSON");

    assert!(normalized(readme).contains("compatibility alone is not enough to justify abstraction"));
    assert!(normalized(release).contains("does not permit registry publication"));
    assert!(release.contains("automatic RALLY consumer migration"));
    assert!(evidence.contains("stop gates block broader abstraction claims"));
    assert!(evidence.contains("stop gates block broader abstraction claims"));
    assert!(consumer.contains("deleted consumer code, added adapter code"));
    assert_eq!(
        gates["registry_publication"],
        "prohibited_by_portfolio_policy"
    );
    assert_eq!(gates["automatic_consumer_migration"], false);
    assert!(normalized(gates["approval_rule"].as_str().unwrap())
        .contains("compatibility evidence is necessary but not sufficient"));

    let rows = gates["gates"]
        .as_array()
        .expect("adoption gates should list consumer rows");
    assert!(rows.len() >= 5);
    for row in rows {
        assert_eq!(row["compatibility_status"], "passed");
        assert_eq!(row["scenarium_expansion_allowed"], false);
        assert!(row["migration_approval"]
            .as_str()
            .is_some_and(|status| !status.eq("approved_by_compatibility")));
    }
    assert!(rows.iter().any(|row| {
        row["consumer"] == "BANISH" && row["migration_approval"] == "rejected_stop_value_exhausted"
    }));
}
