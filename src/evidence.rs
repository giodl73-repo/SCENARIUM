use crate::document::{decode_document, encode_document};
use crate::error::required;
use crate::{ComparisonReport, Error, Provenance, RunRecord};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawArtifactRef")]
pub struct ArtifactRef {
    path: String,
    media_type: String,
    digest: Option<String>,
}

#[derive(Deserialize)]
struct RawArtifactRef {
    path: String,
    media_type: String,
    digest: Option<String>,
}

impl TryFrom<RawArtifactRef> for ArtifactRef {
    type Error = Error;

    fn try_from(raw: RawArtifactRef) -> Result<Self, Self::Error> {
        let mut artifact = Self::new(&raw.path, &raw.media_type)?;
        if let Some(digest) = raw.digest {
            artifact = artifact.with_digest(&digest)?;
        }
        Ok(artifact)
    }
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

    pub fn with_digest(mut self, digest: &str) -> Result<Self, Error> {
        if !valid_sha256(digest) {
            return Err(Error::InvalidDigest(digest.to_string()));
        }
        self.digest = Some(digest.to_string());
        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PacketComparisonRef {
    baseline_run_id: String,
    candidate_run_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawEvidencePacket")]
pub struct EvidencePacket {
    packet_id: String,
    subject: String,
    run_ids: BTreeSet<String>,
    comparisons: BTreeMap<String, PacketComparisonRef>,
    artifacts: BTreeMap<String, ArtifactRef>,
    provenance: Provenance,
}

#[derive(Deserialize)]
struct RawEvidencePacket {
    packet_id: String,
    subject: String,
    #[serde(default)]
    run_ids: BTreeSet<String>,
    #[serde(default)]
    comparisons: BTreeMap<String, PacketComparisonRef>,
    #[serde(default)]
    artifacts: BTreeMap<String, ArtifactRef>,
    provenance: Provenance,
}

impl TryFrom<RawEvidencePacket> for EvidencePacket {
    type Error = Error;

    fn try_from(raw: RawEvidencePacket) -> Result<Self, Self::Error> {
        required(&raw.packet_id, "packet.id")?;
        required(&raw.subject, "packet.subject")?;
        let packet = Self {
            packet_id: raw.packet_id,
            subject: raw.subject,
            run_ids: raw.run_ids,
            comparisons: raw.comparisons,
            artifacts: raw.artifacts,
            provenance: raw.provenance,
        };
        packet.validate()?;
        Ok(packet)
    }
}

impl EvidencePacket {
    pub fn new(packet_id: &str, subject: &str, provenance: Provenance) -> Result<Self, Error> {
        required(packet_id, "packet.id")?;
        required(subject, "packet.subject")?;
        Ok(Self {
            packet_id: packet_id.to_string(),
            subject: subject.to_string(),
            run_ids: BTreeSet::new(),
            comparisons: BTreeMap::new(),
            artifacts: BTreeMap::new(),
            provenance,
        })
    }

    pub fn include_run(&mut self, run: &RunRecord) {
        self.run_ids.insert(run.run_id().to_string());
    }

    pub fn include_comparison(&mut self, comparison: &ComparisonReport) -> Result<(), Error> {
        for run_id in [comparison.baseline_run_id(), comparison.candidate_run_id()] {
            if !self.run_ids.contains(run_id) {
                return Err(Error::MissingPacketRun(run_id.to_string()));
            }
        }
        let id = comparison.comparison_id();
        if self.comparisons.contains_key(id) {
            return Err(Error::DuplicateComparison(id.to_string()));
        }
        self.comparisons.insert(
            id.to_string(),
            PacketComparisonRef {
                baseline_run_id: comparison.baseline_run_id().to_string(),
                candidate_run_id: comparison.candidate_run_id().to_string(),
            },
        );
        Ok(())
    }

    pub fn add_artifact(&mut self, name: &str, artifact: ArtifactRef) -> Result<(), Error> {
        required(name, "artifact.name")?;
        if self.artifacts.contains_key(name) {
            return Err(Error::DuplicateArtifact(name.to_string()));
        }
        self.artifacts.insert(name.to_string(), artifact);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), Error> {
        for run_id in &self.run_ids {
            required(run_id, "packet.run_id")?;
        }
        for (comparison_id, comparison) in &self.comparisons {
            required(comparison_id, "packet.comparison_id")?;
            let expected = format!(
                "{}..{}",
                comparison.baseline_run_id, comparison.candidate_run_id
            );
            if comparison_id != &expected {
                return Err(Error::InvalidRunIdentity);
            }
            for run_id in [&comparison.baseline_run_id, &comparison.candidate_run_id] {
                if !self.run_ids.contains(run_id) {
                    return Err(Error::MissingPacketRun(run_id.clone()));
                }
            }
        }
        for name in self.artifacts.keys() {
            required(name, "packet.artifact.name")?;
        }
        Ok(())
    }

    pub fn to_json(&self) -> Result<String, Error> {
        self.validate()?;
        encode_document(self)
    }

    pub fn from_json(json: &str) -> Result<Self, Error> {
        decode_document(json)
    }
}

fn valid_sha256(digest: &str) -> bool {
    let Some(value) = digest.strip_prefix("sha256:") else {
        return false;
    };
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
