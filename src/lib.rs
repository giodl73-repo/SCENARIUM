mod compare;
mod document;
mod error;
mod evidence;
mod model;
mod seed;

pub use compare::{
    compare_metric_sets, compare_runs, ComparisonReport, ComparisonStatus, MetricDelta,
};
pub use document::{decode_document, encode_document, SCHEMA_VERSION};
pub use error::Error;
pub use evidence::{ArtifactRef, EvidencePacket};
pub use model::{
    Finding, Metric, MetricDirection, Provenance, RunRecord, RunStatus, RunVariant, Scenario,
    Severity,
};
pub use seed::Seed;
