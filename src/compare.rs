use crate::document::{decode_document, encode_document};
use crate::error::required;
use crate::{Error, MetricDirection, RunRecord};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawMetricDelta")]
pub struct MetricDelta {
    metric: String,
    baseline: f64,
    candidate: f64,
    direction: MetricDirection,
    beneficial_change: f64,
}

#[derive(Deserialize)]
struct RawMetricDelta {
    metric: String,
    baseline: f64,
    candidate: f64,
    direction: MetricDirection,
    beneficial_change: f64,
}

impl TryFrom<RawMetricDelta> for MetricDelta {
    type Error = Error;

    fn try_from(raw: RawMetricDelta) -> Result<Self, Self::Error> {
        required(&raw.metric, "comparison.metric")?;
        if !raw.baseline.is_finite()
            || !raw.candidate.is_finite()
            || !raw.beneficial_change.is_finite()
        {
            return Err(Error::NonFiniteDerivedMetric(raw.metric));
        }
        Ok(Self {
            metric: raw.metric,
            baseline: raw.baseline,
            candidate: raw.candidate,
            direction: raw.direction,
            beneficial_change: raw.beneficial_change,
        })
    }
}

impl MetricDelta {
    pub fn metric(&self) -> &str {
        &self.metric
    }

    pub fn beneficial_change(&self) -> f64 {
        self.beneficial_change
    }

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
#[serde(try_from = "RawComparisonReport")]
pub struct ComparisonReport {
    comparison_id: String,
    scenario_id: String,
    baseline_run_id: String,
    candidate_run_id: String,
    deltas: Vec<MetricDelta>,
}

#[derive(Deserialize)]
struct RawComparisonReport {
    comparison_id: String,
    scenario_id: String,
    baseline_run_id: String,
    candidate_run_id: String,
    deltas: Vec<MetricDelta>,
}

impl TryFrom<RawComparisonReport> for ComparisonReport {
    type Error = Error;

    fn try_from(raw: RawComparisonReport) -> Result<Self, Self::Error> {
        required(&raw.scenario_id, "comparison.scenario_id")?;
        required(&raw.baseline_run_id, "comparison.baseline_run_id")?;
        required(&raw.candidate_run_id, "comparison.candidate_run_id")?;
        if raw.deltas.is_empty() {
            return Err(Error::EmptyMetricSet);
        }
        let expected = comparison_id(&raw.baseline_run_id, &raw.candidate_run_id);
        if raw.comparison_id != expected {
            return Err(Error::InvalidRunIdentity);
        }
        Ok(Self {
            comparison_id: raw.comparison_id,
            scenario_id: raw.scenario_id,
            baseline_run_id: raw.baseline_run_id,
            candidate_run_id: raw.candidate_run_id,
            deltas: raw.deltas,
        })
    }
}

impl ComparisonReport {
    pub fn comparison_id(&self) -> &str {
        &self.comparison_id
    }

    pub fn baseline_run_id(&self) -> &str {
        &self.baseline_run_id
    }

    pub fn candidate_run_id(&self) -> &str {
        &self.candidate_run_id
    }

    pub fn deltas(&self) -> &[MetricDelta] {
        &self.deltas
    }

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

    pub fn to_json(&self) -> Result<String, Error> {
        encode_document(self)
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        decode_document(json)
    }
}

pub fn compare_runs(
    baseline: &RunRecord,
    candidate: &RunRecord,
) -> Result<ComparisonReport, Error> {
    if !baseline.variant().is_baseline() {
        return Err(Error::BaselineRequired);
    }
    if !candidate.variant().is_candidate() {
        return Err(Error::CandidateRequired);
    }
    if baseline.scenario_id() != candidate.scenario_id() {
        return Err(Error::ScenarioMismatch {
            baseline: baseline.scenario_id().to_string(),
            candidate: candidate.scenario_id().to_string(),
        });
    }
    if baseline.metrics().is_empty() || candidate.metrics().is_empty() {
        return Err(Error::EmptyMetricSet);
    }
    if baseline.metrics().len() != candidate.metrics().len()
        || baseline.metrics().keys().ne(candidate.metrics().keys())
    {
        return Err(Error::MetricSetMismatch);
    }

    let mut deltas = Vec::with_capacity(baseline.metrics().len());
    for (name, baseline_metric) in baseline.metrics() {
        let candidate_metric = &candidate.metrics()[name];
        if baseline_metric.direction() != candidate_metric.direction() {
            return Err(Error::DirectionMismatch(name.clone()));
        }
        let beneficial_change = match baseline_metric.direction() {
            MetricDirection::HigherIsBetter => candidate_metric.value() - baseline_metric.value(),
            MetricDirection::LowerIsBetter => baseline_metric.value() - candidate_metric.value(),
            MetricDirection::Target(target) => {
                (baseline_metric.value() - target).abs() - (candidate_metric.value() - target).abs()
            }
        };
        if !beneficial_change.is_finite() {
            return Err(Error::NonFiniteDerivedMetric(name.clone()));
        }
        deltas.push(MetricDelta {
            metric: name.clone(),
            baseline: baseline_metric.value(),
            candidate: candidate_metric.value(),
            direction: baseline_metric.direction().clone(),
            beneficial_change,
        });
    }

    Ok(ComparisonReport {
        comparison_id: comparison_id(baseline.run_id(), candidate.run_id()),
        scenario_id: baseline.scenario_id().to_string(),
        baseline_run_id: baseline.run_id().to_string(),
        candidate_run_id: candidate.run_id().to_string(),
        deltas,
    })
}

pub fn compare_metric_sets(
    scenario: &crate::Scenario,
    adapter: &str,
    baseline_variant: crate::RunVariant,
    candidate_variant: crate::RunVariant,
    baseline_metrics: impl IntoIterator<Item = crate::Metric>,
    candidate_metrics: impl IntoIterator<Item = crate::Metric>,
) -> Result<(RunRecord, RunRecord, ComparisonReport), Error> {
    let mut baseline = RunRecord::new(scenario, baseline_variant, adapter)?;
    for metric in baseline_metrics {
        baseline.record_metric(metric)?;
    }
    let mut candidate = RunRecord::new(scenario, candidate_variant, adapter)?;
    for metric in candidate_metrics {
        candidate.record_metric(metric)?;
    }
    let comparison = compare_runs(&baseline, &candidate)?;
    Ok((baseline, candidate, comparison))
}

fn comparison_id(baseline_run_id: &str, candidate_run_id: &str) -> String {
    format!("{baseline_run_id}..{candidate_run_id}")
}
