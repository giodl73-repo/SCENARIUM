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

    assert!(normalized(readme).contains("SCENARIUM owns only the reusable typed contracts"));
    assert!(
        normalized(readme).contains("consumer-specific adapters remain in consumer repositories")
    );
    assert!(plan.contains("SIGNALS retains techniques, skills, scoring rubrics"));
    assert!(roles.contains("Runtime Boundary Engineer"));
    assert!(roles.contains("Consumer Advocate"));
    assert!(validation.contains("consumer metrics stay local"));
}

#[test]
fn comparison_status_does_not_become_recommendation() {
    // Checks SCEN-PF-02: comparison status is not decision authority.
    let readme = include_str!("../README.md");
    let decision_skeptic = include_str!("../.roles/parliament/decision-skeptic.md");
    let contract_tests = include_str!("contract.rs");
    let compare_src = include_str!("../src/compare.rs");

    assert!(readme.contains("No automatic recommendation"));
    assert!(decision_skeptic.contains("Protect the evidence needed to decide not to adopt"));
    assert!(decision_skeptic.contains("Block APIs that hide negative evidence"));
    assert!(contract_tests.contains("ComparisonStatus::Regressed"));
    assert!(contract_tests.contains("ComparisonStatus::Mixed"));
    assert!(contract_tests.contains("ComparisonStatus::Equivalent"));
    assert!(compare_src.contains("ComparisonStatus"));
}

#[test]
fn packet_closure_does_not_prove_evidence_truthfulness() {
    // Checks SCEN-PF-03: packets preserve lineage, not truth.
    let evidence_src = include_str!("../src/evidence.rs");
    let evidence = include_str!("../docs/vtrace/EVIDENCE.md");
    let requirements = include_str!("../docs/vtrace/REQUIREMENTS.md");
    let reviewer = include_str!("../.roles/stakeholders/adversarial-evidence-reviewer.md");
    let pitfalls = include_str!("../.pitfall/scenarium-pitfalls.md");

    assert!(evidence_src.contains("EvidencePacket"));
    assert!(evidence_src.contains("MissingPacketRun"));
    assert!(evidence_src.contains("DuplicateArtifact"));
    assert!(evidence.contains("Foundation evidence supports a crate foundation"));
    assert!(requirements.contains("caller-supplied provenance"));
    assert!(reviewer.contains("trustworthy-looking packet"));
    assert!(reviewer.contains("SCENARIUM-verified facts"));
    assert!(pitfalls.contains("lineage and structure, not truth"));
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
