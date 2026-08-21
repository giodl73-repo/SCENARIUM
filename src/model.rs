use crate::document::{decode_document, encode_document};
use crate::error::required;
use crate::{Error, Seed};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawScenario")]
pub struct Scenario {
    id: String,
    subject: String,
    seed_label: String,
    attributes: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct RawScenario {
    id: String,
    subject: String,
    seed_label: String,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
}

impl TryFrom<RawScenario> for Scenario {
    type Error = Error;

    fn try_from(raw: RawScenario) -> Result<Self, Self::Error> {
        let mut scenario = Self::new(&raw.id, &raw.subject, &raw.seed_label)?;
        for (name, value) in raw.attributes {
            scenario.set_attribute(&name, &value)?;
        }
        Ok(scenario)
    }
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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn seed_label(&self) -> &str {
        &self.seed_label
    }

    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<(), Error> {
        required(name, "scenario.attribute.name")?;
        required(value, "scenario.attribute.value")?;
        self.attributes.insert(name.to_string(), value.to_string());
        Ok(())
    }

    pub fn seed(&self) -> Seed {
        Seed::from_label(&format!("{}:{}", self.id, self.seed_label))
    }

    pub fn to_json(&self) -> Result<String, Error> {
        encode_document(self)
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        decode_document(json)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawRunVariant")]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum RunVariant {
    Inertia,
    Baseline(String),
    Candidate(String),
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
enum RawRunVariant {
    Inertia,
    Baseline(String),
    Candidate(String),
}

impl TryFrom<RawRunVariant> for RunVariant {
    type Error = Error;

    fn try_from(raw: RawRunVariant) -> Result<Self, Self::Error> {
        match raw {
            RawRunVariant::Inertia => Ok(Self::Inertia),
            RawRunVariant::Baseline(id) => Self::baseline(&id),
            RawRunVariant::Candidate(id) => Self::candidate(&id),
        }
    }
}

impl RunVariant {
    pub fn baseline(id: &str) -> Result<Self, Error> {
        required(id, "run.variant.id")?;
        Ok(Self::Baseline(id.to_string()))
    }

    pub fn candidate(id: &str) -> Result<Self, Error> {
        required(id, "run.variant.id")?;
        Ok(Self::Candidate(id.to_string()))
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Inertia => "inertia",
            Self::Baseline(id) | Self::Candidate(id) => id,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Inertia => "inertia",
            Self::Baseline(_) => "baseline",
            Self::Candidate(_) => "candidate",
        }
    }

    pub fn is_baseline(&self) -> bool {
        matches!(self, Self::Inertia | Self::Baseline(_))
    }

    pub fn is_candidate(&self) -> bool {
        matches!(self, Self::Candidate(_))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawMetricDirection")]
#[serde(tag = "kind", content = "target", rename_all = "snake_case")]
pub enum MetricDirection {
    HigherIsBetter,
    LowerIsBetter,
    Target(f64),
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "target", rename_all = "snake_case")]
enum RawMetricDirection {
    HigherIsBetter,
    LowerIsBetter,
    Target(f64),
}

impl TryFrom<RawMetricDirection> for MetricDirection {
    type Error = Error;

    fn try_from(raw: RawMetricDirection) -> Result<Self, Self::Error> {
        match raw {
            RawMetricDirection::HigherIsBetter => Ok(Self::HigherIsBetter),
            RawMetricDirection::LowerIsBetter => Ok(Self::LowerIsBetter),
            RawMetricDirection::Target(target) if target.is_finite() => Ok(Self::Target(target)),
            RawMetricDirection::Target(_) => Err(Error::NonFiniteMetric("target".to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawMetric")]
pub struct Metric {
    name: String,
    value: f64,
    direction: MetricDirection,
}

#[derive(Deserialize)]
struct RawMetric {
    name: String,
    value: f64,
    direction: MetricDirection,
}

impl TryFrom<RawMetric> for Metric {
    type Error = Error;

    fn try_from(raw: RawMetric) -> Result<Self, Self::Error> {
        Self::new(&raw.name, raw.value, raw.direction)
    }
}

impl Metric {
    pub fn new(name: &str, value: f64, direction: MetricDirection) -> Result<Self, Error> {
        required(name, "metric.name")?;
        if !value.is_finite() {
            return Err(Error::NonFiniteMetric(name.to_string()));
        }
        if let MetricDirection::Target(target) = direction {
            if !target.is_finite() {
                return Err(Error::NonFiniteMetric(name.to_string()));
            }
        }
        Ok(Self {
            name: name.to_string(),
            value,
            direction,
        })
    }

    pub fn toward_target(name: &str, value: f64, target: f64) -> Result<Self, Error> {
        Self::new(name, value, MetricDirection::Target(target))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn direction(&self) -> &MetricDirection {
        &self.direction
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
#[serde(try_from = "RawFinding")]
pub struct Finding {
    severity: Severity,
    code: String,
    location: String,
    message: String,
}

#[derive(Deserialize)]
struct RawFinding {
    severity: Severity,
    code: String,
    location: String,
    message: String,
}

impl TryFrom<RawFinding> for Finding {
    type Error = Error;

    fn try_from(raw: RawFinding) -> Result<Self, Self::Error> {
        Self::new(raw.severity, &raw.code, &raw.location, &raw.message)
    }
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

    pub fn severity(&self) -> Severity {
        self.severity
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawProvenance")]
pub struct Provenance {
    producer: String,
    producer_version: String,
    input_id: String,
}

#[derive(Deserialize)]
struct RawProvenance {
    producer: String,
    producer_version: String,
    input_id: String,
}

impl TryFrom<RawProvenance> for Provenance {
    type Error = Error;

    fn try_from(raw: RawProvenance) -> Result<Self, Self::Error> {
        Self::new(&raw.producer, &raw.producer_version, &raw.input_id)
    }
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

    pub fn producer(&self) -> &str {
        &self.producer
    }

    pub fn producer_version(&self) -> &str {
        &self.producer_version
    }

    pub fn input_id(&self) -> &str {
        &self.input_id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawRunRecord")]
pub struct RunRecord {
    run_id: String,
    scenario_id: String,
    seed_label: String,
    variant: RunVariant,
    adapter: String,
    metrics: BTreeMap<String, Metric>,
    findings: Vec<Finding>,
    provenance: Option<Provenance>,
}

#[derive(Deserialize)]
struct RawRunRecord {
    run_id: String,
    scenario_id: String,
    seed_label: String,
    variant: RunVariant,
    adapter: String,
    #[serde(default)]
    metrics: BTreeMap<String, Metric>,
    #[serde(default)]
    findings: Vec<Finding>,
    provenance: Option<Provenance>,
}

impl TryFrom<RawRunRecord> for RunRecord {
    type Error = Error;

    fn try_from(raw: RawRunRecord) -> Result<Self, Self::Error> {
        required(&raw.scenario_id, "run.scenario_id")?;
        required(&raw.seed_label, "run.seed_label")?;
        required(&raw.adapter, "run.adapter")?;
        let expected = run_id(
            &raw.adapter,
            &raw.scenario_id,
            &raw.seed_label,
            &raw.variant,
        );
        if raw.run_id != expected {
            return Err(Error::InvalidRunIdentity);
        }
        for (name, metric) in &raw.metrics {
            if name != metric.name() {
                return Err(Error::DuplicateMetric(name.clone()));
            }
        }
        Ok(Self {
            run_id: raw.run_id,
            scenario_id: raw.scenario_id,
            seed_label: raw.seed_label,
            variant: raw.variant,
            adapter: raw.adapter,
            metrics: raw.metrics,
            findings: raw.findings,
            provenance: raw.provenance,
        })
    }
}

impl RunRecord {
    pub fn new(scenario: &Scenario, variant: RunVariant, adapter: &str) -> Result<Self, Error> {
        required(adapter, "run.adapter")?;
        Ok(Self {
            run_id: run_id(adapter, scenario.id(), scenario.seed_label(), &variant),
            scenario_id: scenario.id().to_string(),
            seed_label: scenario.seed_label().to_string(),
            variant,
            adapter: adapter.to_string(),
            metrics: BTreeMap::new(),
            findings: Vec::new(),
            provenance: None,
        })
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn variant(&self) -> &RunVariant {
        &self.variant
    }

    pub fn metrics(&self) -> &BTreeMap<String, Metric> {
        &self.metrics
    }

    pub fn record_metric(&mut self, metric: Metric) -> Result<(), Error> {
        if self.metrics.contains_key(metric.name()) {
            return Err(Error::DuplicateMetric(metric.name().to_string()));
        }
        self.metrics.insert(metric.name().to_string(), metric);
        Ok(())
    }

    pub fn add_finding(&mut self, finding: Finding) {
        self.findings.push(finding);
    }

    pub fn set_provenance(&mut self, provenance: Provenance) {
        self.provenance = Some(provenance);
    }

    pub fn status(&self) -> RunStatus {
        if self
            .findings
            .iter()
            .any(|finding| finding.severity() == Severity::Error)
        {
            RunStatus::Error
        } else if self.findings.is_empty() {
            RunStatus::Pass
        } else {
            RunStatus::Review
        }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        encode_document(self)
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        decode_document(json)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Pass,
    Review,
    Error,
}

fn run_id(adapter: &str, scenario_id: &str, seed_label: &str, variant: &RunVariant) -> String {
    let parts = [
        adapter,
        scenario_id,
        seed_label,
        variant.kind(),
        variant.id(),
    ];
    let encoded = parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|");
    format!("run:{encoded}")
}
