use scenarium::{
    compare_runs, decode_document, ArtifactRef, ComparisonStatus, Error, EvidencePacket, Finding,
    Metric, MetricDirection, Provenance, RunRecord, RunStatus, RunVariant, Scenario, Seed,
    Severity, SCHEMA_VERSION,
};

fn run_pair(seed_label: &str) -> (RunRecord, RunRecord) {
    let scenario = Scenario::new("checkout", "Checkout reliability", seed_label).unwrap();
    let inertia = RunRecord::new(&scenario, RunVariant::Inertia, "fixture").unwrap();
    let candidate = RunRecord::new(
        &scenario,
        RunVariant::candidate("retry-v1").unwrap(),
        "fixture",
    )
    .unwrap();
    (inertia, candidate)
}

fn add_metric(run: &mut RunRecord, name: &str, value: f64, direction: MetricDirection) {
    run.record_metric(Metric::new(name, value, direction).unwrap())
        .unwrap();
}

#[test]
fn seeds_are_repeatable_distinct_and_bounded() {
    let mut left = Seed::from_label("same");
    let mut right = Seed::from_label("same");
    assert_eq!(
        (0..8).map(|_| left.next_u32()).collect::<Vec<_>>(),
        (0..8).map(|_| right.next_u32()).collect::<Vec<_>>()
    );

    let mut zero = Seed::from_u64(0);
    let mut one = Seed::from_u64(1);
    assert_ne!(zero.next_u32(), one.next_u32());

    let mut bounded = Seed::from_label("bounded");
    assert!((0..10_000).all(|_| bounded.next_bounded(7) < 7));
}

#[test]
fn run_identity_includes_seed_and_variant_kind() {
    let (first, _) = run_pair("seed-1");
    let (second, _) = run_pair("seed-2");
    assert_ne!(first.run_id(), second.run_id());

    let scenario = Scenario::new("checkout", "Checkout reliability", "seed-1").unwrap();
    let baseline =
        RunRecord::new(&scenario, RunVariant::baseline("same").unwrap(), "fixture").unwrap();
    let candidate =
        RunRecord::new(&scenario, RunVariant::candidate("same").unwrap(), "fixture").unwrap();
    assert_ne!(baseline.run_id(), candidate.run_id());
}

#[test]
fn comparison_covers_all_outcomes() {
    let cases = [
        ((0.5, 0.8), (100.0, 80.0), ComparisonStatus::Improved),
        ((0.8, 0.5), (80.0, 100.0), ComparisonStatus::Regressed),
        ((0.5, 0.8), (80.0, 100.0), ComparisonStatus::Mixed),
        ((0.5, 0.5), (80.0, 80.0), ComparisonStatus::Equivalent),
    ];
    for (completion, latency, expected) in cases {
        let (mut baseline, mut candidate) = run_pair("seed");
        add_metric(
            &mut baseline,
            "completion",
            completion.0,
            MetricDirection::HigherIsBetter,
        );
        add_metric(
            &mut candidate,
            "completion",
            completion.1,
            MetricDirection::HigherIsBetter,
        );
        add_metric(
            &mut baseline,
            "latency",
            latency.0,
            MetricDirection::LowerIsBetter,
        );
        add_metric(
            &mut candidate,
            "latency",
            latency.1,
            MetricDirection::LowerIsBetter,
        );
        assert_eq!(
            compare_runs(&baseline, &candidate).unwrap().status(),
            expected
        );
    }
}

#[test]
fn comparison_rejects_empty_wrong_variant_and_overflow() {
    let (baseline, candidate) = run_pair("seed");
    assert_eq!(
        compare_runs(&baseline, &candidate),
        Err(Error::EmptyMetricSet)
    );

    let scenario = Scenario::new("checkout", "Checkout reliability", "seed").unwrap();
    let other_baseline = RunRecord::new(
        &scenario,
        RunVariant::baseline("control").unwrap(),
        "fixture",
    )
    .unwrap();
    assert_eq!(
        compare_runs(&baseline, &other_baseline),
        Err(Error::CandidateRequired)
    );

    let (mut huge_baseline, mut huge_candidate) = run_pair("huge");
    add_metric(
        &mut huge_baseline,
        "range",
        -f64::MAX,
        MetricDirection::HigherIsBetter,
    );
    add_metric(
        &mut huge_candidate,
        "range",
        f64::MAX,
        MetricDirection::HigherIsBetter,
    );
    assert_eq!(
        compare_runs(&huge_baseline, &huge_candidate),
        Err(Error::NonFiniteDerivedMetric("range".to_string()))
    );
}

#[test]
fn target_metrics_require_a_finite_target() {
    assert_eq!(
        Metric::toward_target("distance", 1.0, f64::INFINITY),
        Err(Error::NonFiniteMetric("distance".to_string()))
    );
    let metric = Metric::toward_target("distance", 8.0, 10.0).unwrap();
    assert_eq!(metric.direction(), &MetricDirection::Target(10.0));
}

#[test]
fn findings_drive_structured_run_status() {
    let (mut run, _) = run_pair("seed");
    assert_eq!(run.status(), RunStatus::Pass);
    run.add_finding(Finding::new(Severity::Warning, "S-WARN-001", "run", "review this").unwrap());
    assert_eq!(run.status(), RunStatus::Review);
    run.add_finding(Finding::new(Severity::Error, "S-ERR-001", "run", "invalid").unwrap());
    assert_eq!(run.status(), RunStatus::Error);
}

#[test]
fn versioned_decode_rejects_invalid_records_and_schemas() {
    let scenario = Scenario::new("checkout", "Checkout reliability", "seed").unwrap();
    let json = scenario.to_json().unwrap();
    assert_eq!(Scenario::from_json(&json).unwrap(), scenario);

    let wrong_schema = json.replace(SCHEMA_VERSION, "scenarium.v999");
    assert!(matches!(
        Scenario::from_json(&wrong_schema),
        Err(Error::InvalidSchema { .. })
    ));

    let invalid = format!(
        r#"{{"schema":"{SCHEMA_VERSION}","payload":{{"id":"","subject":"x","seed_label":"s","attributes":{{}}}}}}"#
    );
    assert!(matches!(Scenario::from_json(&invalid), Err(Error::Json(_))));

    let invalid_variant =
        format!(r#"{{"schema":"{SCHEMA_VERSION}","payload":{{"kind":"candidate","id":""}}}}"#);
    assert!(decode_document::<RunVariant>(&invalid_variant).is_err());
}

#[test]
fn retained_v1_fixture_round_trips_canonically() {
    let fixture = include_str!("fixtures/scenario.v1.json");
    let scenario = Scenario::from_json(fixture).unwrap();
    assert_eq!(scenario.id(), "checkout");
    assert_eq!(
        scenario.to_json().unwrap().replace("\r\n", "\n"),
        fixture.trim().replace("\r\n", "\n")
    );
}

#[test]
fn packet_is_canonical_append_only_and_reference_closed() {
    let (mut baseline, mut candidate) = run_pair("seed");
    add_metric(
        &mut baseline,
        "completion",
        0.5,
        MetricDirection::HigherIsBetter,
    );
    add_metric(
        &mut candidate,
        "completion",
        0.8,
        MetricDirection::HigherIsBetter,
    );
    let comparison = compare_runs(&baseline, &candidate).unwrap();
    let provenance = Provenance::new("signals", "1.0.0", "checkout-spec").unwrap();
    let mut packet = EvidencePacket::new("packet", "Checkout", provenance).unwrap();

    assert_eq!(
        packet.include_comparison(&comparison),
        Err(Error::MissingPacketRun(baseline.run_id().to_string()))
    );

    packet.include_run(&candidate);
    packet.include_run(&baseline);
    packet.include_comparison(&comparison).unwrap();
    packet
        .add_artifact(
            "comparison",
            ArtifactRef::new("comparison.json", "application/json")
                .unwrap()
                .with_digest(
                    "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                )
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        packet.add_artifact(
            "comparison",
            ArtifactRef::new("replacement.json", "application/json").unwrap(),
        ),
        Err(Error::DuplicateArtifact("comparison".to_string()))
    );

    let json = packet.to_json().unwrap();
    assert_eq!(EvidencePacket::from_json(&json).unwrap(), packet);
    assert_eq!(packet.to_json().unwrap(), packet.to_json().unwrap());
}

#[test]
fn invalid_digest_is_rejected() {
    assert!(matches!(
        ArtifactRef::new("artifact", "application/json")
            .unwrap()
            .with_digest("sha256:ABC"),
        Err(Error::InvalidDigest(_))
    ));
}
