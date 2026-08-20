use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    EmptyField(&'static str),
    NonFiniteMetric(String),
    DuplicateMetric(String),
    ScenarioMismatch { baseline: String, candidate: String },
    MetricSetMismatch,
    DirectionMismatch(String),
    BaselineRequired,
    DuplicateArtifact(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(f, "{field} must not be empty"),
            Self::NonFiniteMetric(metric) => {
                write!(f, "metric {metric} must have a finite value")
            }
            Self::DuplicateMetric(metric) => write!(f, "metric {metric} already exists"),
            Self::ScenarioMismatch {
                baseline,
                candidate,
            } => write!(
                f,
                "cannot compare different scenarios: {baseline} and {candidate}"
            ),
            Self::MetricSetMismatch => write!(f, "baseline and candidate metric sets differ"),
            Self::DirectionMismatch(metric) => {
                write!(f, "metric {metric} uses different comparison directions")
            }
            Self::BaselineRequired => write!(f, "the first run must be a baseline"),
            Self::DuplicateArtifact(name) => {
                write!(f, "artifact {name} already exists; packets are append-only")
            }
        }
    }
}

impl StdError for Error {}

fn required(value: &str, field: &'static str) -> Result<(), Error> {
    if value.trim().is_empty() {
        Err(Error::EmptyField(field))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seed {
    state: u64,
}

impl Seed {
    pub fn from_label(label: &str) -> Self {
        let mut state = 0xcbf2_9ce4_8422_2325u64;
        for byte in label.bytes() {
            state ^= u64::from(byte);
            state = state.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self { state }
    }

    pub fn from_u64(value: u64) -> Self {
        Self {
            state: if value == 0 { 1 } else { value },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.state >> 32) as u32
    }

    pub fn next_bounded(&mut self, upper_exclusive: u32) -> u32 {
        if upper_exclusive == 0 {
            0
        } else {
            self.next_u32() % upper_exclusive
        }
    }

    pub fn choose_index(&mut self, len: usize) -> Option<usize> {
        (len > 0).then(|| self.next_bounded(len as u32) as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub subject: String,
    pub seed_label: String,
    pub attributes: BTreeMap<String, String>,
}

impl Scenario {
    pub fn new(id: &str, subject: &str, seed_label: &str) -> Result<Self, Error> {
        required(id, "scenario.id")?;
        required(subject, "scenario.subject")?;
        required(seed_label, "scenario.seed_label")?;
        Ok(Self {
            id: id.to_string(),
            subject: subject.to_string(),
            seed_label: seed_label.to_string(),
            attributes: BTreeMap::new(),
        })
    }

    pub fn seed(&self) -> Seed {
        Seed::from_label(&format!("{}:{}", self.id, self.seed_label))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum RunVariant {
    Inertia,
    Baseline(String),
    Candidate(String),
}

impl RunVariant {
    pub fn id(&self) -> &str {
        match self {
            Self::Inertia => "inertia",
            Self::Baseline(id) | Self::Candidate(id) => id,
        }
    }

    pub fn is_baseline(&self) -> bool {
        matches!(self, Self::Inertia | Self::Baseline(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricDirection {
    HigherIsBetter,
    LowerIsBetter,
    Target,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub direction: MetricDirection,
    pub target: Option<f64>,
}

impl Metric {
    pub fn new(name: &str, value: f64, direction: MetricDirection) -> Result<Self, Error> {
        required(name, "metric.name")?;
        if !value.is_finite() {
            return Err(Error::NonFiniteMetric(name.to_string()));
        }
        Ok(Self {
            name: name.to_string(),
            value,
            direction,
            target: None,
        })
    }

    pub fn toward_target(name: &str, value: f64, target: f64) -> Result<Self, Error> {
        if !target.is_finite() {
            return Err(Error::NonFiniteMetric(name.to_string()));
        }
        let mut metric = Self::new(name, value, MetricDirection::Target)?;
        metric.target = Some(target);
        Ok(metric)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Note,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub code: String,
    pub location: String,
    pub message: String,
}

impl Finding {
    pub fn new(
        severity: Severity,
        code: &str,
        location: &str,
        message: &str,
    ) -> Result<Self, Error> {
        required(code, "finding.code")?;
        required(location, "finding.location")?;
        required(message, "finding.message")?;
        Ok(Self {
            severity,
            code: code.to_string(),
            location: location.to_string(),
            message: message.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub producer: String,
    pub producer_version: String,
    pub input_id: String,
}

impl Provenance {
    pub fn new(producer: &str, producer_version: &str, input_id: &str) -> Result<Self, Error> {
        required(producer, "provenance.producer")?;
        required(producer_version, "provenance.producer_version")?;
        required(input_id, "provenance.input_id")?;
        Ok(Self {
            producer: producer.to_string(),
            producer_version: producer_version.to_string(),
            input_id: input_id.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: String,
    pub scenario_id: String,
    pub seed_label: String,
    pub variant: RunVariant,
    pub adapter: String,
    pub metrics: BTreeMap<String, Metric>,
    pub findings: Vec<Finding>,
    pub provenance: Option<Provenance>,
}

impl RunRecord {
    pub fn new(scenario: &Scenario, variant: RunVariant, adapter: &str) -> Result<Self, Error> {
        required(adapter, "run.adapter")?;
        required(variant.id(), "run.variant")?;
        Ok(Self {
            run_id: format!("{}:{}:{}", adapter, scenario.id, variant.id()),
            scenario_id: scenario.id.clone(),
            seed_label: scenario.seed_label.clone(),
            variant,
            adapter: adapter.to_string(),
            metrics: BTreeMap::new(),
            findings: Vec::new(),
            provenance: None,
        })
    }

    pub fn record_metric(&mut self, metric: Metric) -> Result<(), Error> {
        if self.metrics.contains_key(&metric.name) {
            return Err(Error::DuplicateMetric(metric.name));
        }
        self.metrics.insert(metric.name.clone(), metric);
        Ok(())
    }

    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    pub fn status(&self) -> RunStatus {
        if self
            .findings
            .iter()
            .any(|finding| finding.severity == Severity::Error)
        {
            RunStatus::Error
        } else if self.findings.is_empty() {
            RunStatus::Pass
        } else {
            RunStatus::Review
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Pass,
    Review,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricDelta {
    pub metric: String,
    pub baseline: f64,
    pub candidate: f64,
    pub direction: MetricDirection,
    pub target: Option<f64>,
    pub beneficial_change: f64,
}

impl MetricDelta {
    pub fn improved(&self) -> bool {
        self.beneficial_change > 0.0
    }

    pub fn regressed(&self) -> bool {
        self.beneficial_change < 0.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonStatus {
    Improved,
    Regressed,
    Mixed,
    Equivalent,
}

impl ComparisonStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Improved => "improved",
            Self::Regressed => "regressed",
            Self::Mixed => "mixed",
            Self::Equivalent => "equivalent",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonReport {
    pub comparison_id: String,
    pub scenario_id: String,
    pub baseline_run_id: String,
    pub candidate_run_id: String,
    pub deltas: Vec<MetricDelta>,
}

impl ComparisonReport {
    pub fn status(&self) -> ComparisonStatus {
        let improved = self.deltas.iter().any(MetricDelta::improved);
        let regressed = self.deltas.iter().any(MetricDelta::regressed);
        match (improved, regressed) {
            (true, true) => ComparisonStatus::Mixed,
            (true, false) => ComparisonStatus::Improved,
            (false, true) => ComparisonStatus::Regressed,
            (false, false) => ComparisonStatus::Equivalent,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

pub fn compare_runs(
    baseline: &RunRecord,
    candidate: &RunRecord,
) -> Result<ComparisonReport, Error> {
    if !baseline.variant.is_baseline() {
        return Err(Error::BaselineRequired);
    }
    if baseline.scenario_id != candidate.scenario_id {
        return Err(Error::ScenarioMismatch {
            baseline: baseline.scenario_id.clone(),
            candidate: candidate.scenario_id.clone(),
        });
    }
    if baseline.metrics.len() != candidate.metrics.len()
        || baseline.metrics.keys().ne(candidate.metrics.keys())
    {
        return Err(Error::MetricSetMismatch);
    }

    let mut deltas = Vec::with_capacity(baseline.metrics.len());
    for (name, baseline_metric) in &baseline.metrics {
        let candidate_metric = &candidate.metrics[name];
        if baseline_metric.direction != candidate_metric.direction
            || baseline_metric.target != candidate_metric.target
        {
            return Err(Error::DirectionMismatch(name.clone()));
        }
        let beneficial_change = match baseline_metric.direction {
            MetricDirection::HigherIsBetter => candidate_metric.value - baseline_metric.value,
            MetricDirection::LowerIsBetter => baseline_metric.value - candidate_metric.value,
            MetricDirection::Target => {
                let target = baseline_metric.target.unwrap_or(0.0);
                (baseline_metric.value - target).abs() - (candidate_metric.value - target).abs()
            }
        };
        deltas.push(MetricDelta {
            metric: name.clone(),
            baseline: baseline_metric.value,
            candidate: candidate_metric.value,
            direction: baseline_metric.direction,
            target: baseline_metric.target,
            beneficial_change,
        });
    }

    Ok(ComparisonReport {
        comparison_id: format!("{}..{}", baseline.run_id, candidate.run_id),
        scenario_id: baseline.scenario_id.clone(),
        baseline_run_id: baseline.run_id.clone(),
        candidate_run_id: candidate.run_id.clone(),
        deltas,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub path: String,
    pub media_type: String,
    pub digest: Option<String>,
}

impl ArtifactRef {
    pub fn new(path: &str, media_type: &str) -> Result<Self, Error> {
        required(path, "artifact.path")?;
        required(media_type, "artifact.media_type")?;
        Ok(Self {
            path: path.to_string(),
            media_type: media_type.to_string(),
            digest: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidencePacket {
    pub packet_id: String,
    pub subject: String,
    pub run_ids: Vec<String>,
    pub comparison_ids: Vec<String>,
    pub artifacts: BTreeMap<String, ArtifactRef>,
    pub provenance: Provenance,
}

impl EvidencePacket {
    pub fn new(packet_id: &str, subject: &str, provenance: Provenance) -> Result<Self, Error> {
        required(packet_id, "packet.id")?;
        required(subject, "packet.subject")?;
        Ok(Self {
            packet_id: packet_id.to_string(),
            subject: subject.to_string(),
            run_ids: Vec::new(),
            comparison_ids: Vec::new(),
            artifacts: BTreeMap::new(),
            provenance,
        })
    }

    pub fn include_run(&mut self, run: &RunRecord) {
        if !self.run_ids.contains(&run.run_id) {
            self.run_ids.push(run.run_id.clone());
        }
    }

    pub fn include_comparison(&mut self, comparison: &ComparisonReport) {
        if !self.comparison_ids.contains(&comparison.comparison_id) {
            self.comparison_ids.push(comparison.comparison_id.clone());
        }
    }

    pub fn add_artifact(&mut self, name: &str, artifact: ArtifactRef) -> Result<(), Error> {
        required(name, "artifact.name")?;
        if self.artifacts.contains_key(name) {
            return Err(Error::DuplicateArtifact(name.to_string()));
        }
        self.artifacts.insert(name.to_string(), artifact);
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_pair() -> (RunRecord, RunRecord) {
        let scenario = Scenario::new("checkout", "Checkout reliability", "seed-1").unwrap();
        let inertia = RunRecord::new(&scenario, RunVariant::Inertia, "fixture").unwrap();
        let candidate = RunRecord::new(
            &scenario,
            RunVariant::Candidate("retry-v1".to_string()),
            "fixture",
        )
        .unwrap();
        (inertia, candidate)
    }

    #[test]
    fn seed_is_repeatable() {
        let mut left = Seed::from_label("same");
        let mut right = Seed::from_label("same");
        assert_eq!(
            (0..8).map(|_| left.next_u32()).collect::<Vec<_>>(),
            (0..8).map(|_| right.next_u32()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn inertia_comparison_reports_improvement() {
        let (mut inertia, mut candidate) = run_pair();
        inertia
            .record_metric(
                Metric::new("completion_rate", 0.61, MetricDirection::HigherIsBetter).unwrap(),
            )
            .unwrap();
        candidate
            .record_metric(
                Metric::new("completion_rate", 0.83, MetricDirection::HigherIsBetter).unwrap(),
            )
            .unwrap();
        inertia
            .record_metric(
                Metric::new("latency_ms", 410.0, MetricDirection::LowerIsBetter).unwrap(),
            )
            .unwrap();
        candidate
            .record_metric(
                Metric::new("latency_ms", 350.0, MetricDirection::LowerIsBetter).unwrap(),
            )
            .unwrap();

        let report = compare_runs(&inertia, &candidate).unwrap();
        assert_eq!(report.status(), ComparisonStatus::Improved);
        assert!(report.deltas.iter().all(MetricDelta::improved));
    }

    #[test]
    fn mixed_outcomes_remain_visible() {
        let (mut inertia, mut candidate) = run_pair();
        inertia
            .record_metric(
                Metric::new("completion_rate", 0.61, MetricDirection::HigherIsBetter).unwrap(),
            )
            .unwrap();
        candidate
            .record_metric(
                Metric::new("completion_rate", 0.83, MetricDirection::HigherIsBetter).unwrap(),
            )
            .unwrap();
        inertia
            .record_metric(
                Metric::new("latency_ms", 410.0, MetricDirection::LowerIsBetter).unwrap(),
            )
            .unwrap();
        candidate
            .record_metric(
                Metric::new("latency_ms", 450.0, MetricDirection::LowerIsBetter).unwrap(),
            )
            .unwrap();

        assert_eq!(
            compare_runs(&inertia, &candidate).unwrap().status(),
            ComparisonStatus::Mixed
        );
    }

    #[test]
    fn invalid_and_duplicate_metrics_are_structured_failures() {
        assert_eq!(
            Metric::new("score", f64::NAN, MetricDirection::HigherIsBetter),
            Err(Error::NonFiniteMetric("score".to_string()))
        );

        let (mut inertia, _) = run_pair();
        let metric = Metric::new("score", 1.0, MetricDirection::HigherIsBetter).unwrap();
        inertia.record_metric(metric.clone()).unwrap();
        assert_eq!(
            inertia.record_metric(metric),
            Err(Error::DuplicateMetric("score".to_string()))
        );
    }

    #[test]
    fn packet_rejects_artifact_replacement() {
        let provenance = Provenance::new("signals", "1.0.0", "checkout-spec").unwrap();
        let mut packet =
            EvidencePacket::new("checkout-evidence", "Checkout reliability", provenance).unwrap();
        packet
            .add_artifact(
                "comparison",
                ArtifactRef::new("comparison.json", "application/json").unwrap(),
            )
            .unwrap();

        assert_eq!(
            packet.add_artifact(
                "comparison",
                ArtifactRef::new("replacement.json", "application/json").unwrap(),
            ),
            Err(Error::DuplicateArtifact("comparison".to_string()))
        );
    }

    #[test]
    fn packet_json_is_deterministic() {
        let provenance = Provenance::new("scenarium", "0.1.0", "fixture").unwrap();
        let mut packet = EvidencePacket::new("packet-1", "Fixture", provenance).unwrap();
        packet
            .add_artifact("z", ArtifactRef::new("z.json", "application/json").unwrap())
            .unwrap();
        packet
            .add_artifact("a", ArtifactRef::new("a.json", "application/json").unwrap())
            .unwrap();

        assert_eq!(packet.to_json().unwrap(), packet.to_json().unwrap());
        assert!(
            packet.to_json().unwrap().find("\"a\"").unwrap()
                < packet.to_json().unwrap().find("\"z\"").unwrap()
        );
    }
}
