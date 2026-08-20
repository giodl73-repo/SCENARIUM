use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    EmptyField(&'static str),
    InvalidSchema { expected: String, actual: String },
    Json(String),
    NonFiniteMetric(String),
    NonFiniteDerivedMetric(String),
    DuplicateMetric(String),
    ScenarioMismatch { baseline: String, candidate: String },
    MetricSetMismatch,
    EmptyMetricSet,
    DirectionMismatch(String),
    BaselineRequired,
    CandidateRequired,
    InvalidRunIdentity,
    DuplicateArtifact(String),
    DuplicateComparison(String),
    InvalidDigest(String),
    MissingPacketRun(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField(field) => write!(f, "{field} must not be empty"),
            Self::InvalidSchema { expected, actual } => {
                write!(f, "unsupported schema {actual}; expected {expected}")
            }
            Self::Json(message) => write!(f, "invalid JSON document: {message}"),
            Self::NonFiniteMetric(metric) => {
                write!(f, "metric {metric} must have finite values")
            }
            Self::NonFiniteDerivedMetric(metric) => {
                write!(f, "metric {metric} produced a non-finite comparison delta")
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
            Self::EmptyMetricSet => write!(f, "comparison requires at least one metric"),
            Self::DirectionMismatch(metric) => {
                write!(f, "metric {metric} uses different comparison directions")
            }
            Self::BaselineRequired => write!(f, "the first run must be a baseline"),
            Self::CandidateRequired => write!(f, "the second run must be a candidate"),
            Self::InvalidRunIdentity => write!(f, "run identity does not match its inputs"),
            Self::DuplicateArtifact(name) => {
                write!(f, "artifact {name} already exists; packets are append-only")
            }
            Self::DuplicateComparison(id) => {
                write!(f, "comparison {id} already exists in the packet")
            }
            Self::InvalidDigest(digest) => {
                write!(
                    f,
                    "digest {digest} must use sha256:<64 lowercase hex characters>"
                )
            }
            Self::MissingPacketRun(id) => {
                write!(f, "packet comparison references missing run {id}")
            }
        }
    }
}

impl StdError for Error {}

pub(crate) fn required(value: &str, field: &'static str) -> Result<(), Error> {
    if value.trim().is_empty() {
        Err(Error::EmptyField(field))
    } else {
        Ok(())
    }
}
