use scenarium::{ArtifactRef, Error, EvidencePacket, Provenance, Scenario};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

const SIGNALS_SCHEMA: &str = "signals.artifact.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SignalsSidecar {
    schema: String,
    artifact_id: String,
    skill: String,
    topic: String,
    item: String,
    date: String,
    skill_version: String,
    input: String,
    artifact_path: String,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

struct SignalsBridge {
    scenario: Scenario,
    packet: EvidencePacket,
    extensions: BTreeMap<String, Value>,
}

fn to_bridge(sidecar: &SignalsSidecar) -> Result<SignalsBridge, Error> {
    if sidecar.schema != SIGNALS_SCHEMA {
        return Err(Error::InvalidSchema {
            expected: SIGNALS_SCHEMA.to_string(),
            actual: sidecar.schema.clone(),
        });
    }

    let mut scenario = Scenario::new(&sidecar.artifact_id, &sidecar.topic, &sidecar.date)?;
    scenario.set_attribute("skill", &sidecar.skill)?;
    scenario.set_attribute("item", &sidecar.item)?;
    scenario.set_attribute("skill_version", &sidecar.skill_version)?;
    scenario.set_attribute("input", &sidecar.input)?;

    let provenance = Provenance::new(&sidecar.skill, &sidecar.skill_version, &sidecar.input)?;
    let mut packet = EvidencePacket::new(
        &sidecar.artifact_id,
        &format!("{}:{}", sidecar.topic, sidecar.item),
        provenance,
    )?;
    packet.add_artifact(
        "primary",
        ArtifactRef::new(&sidecar.artifact_path, "text/markdown")?,
    )?;

    Ok(SignalsBridge {
        scenario,
        packet,
        extensions: sidecar.extensions.clone(),
    })
}

fn from_bridge(bridge: &SignalsBridge) -> Result<SignalsSidecar, Error> {
    let attributes = bridge.scenario.attributes();
    let skill = attributes
        .get("skill")
        .cloned()
        .ok_or(Error::EmptyField("signals.skill"))?;
    let item = attributes
        .get("item")
        .cloned()
        .ok_or(Error::EmptyField("signals.item"))?;
    let artifact = bridge
        .packet
        .artifacts()
        .get("primary")
        .ok_or(Error::EmptyField("signals.artifact_path"))?;
    let provenance = bridge.packet.provenance();

    Ok(SignalsSidecar {
        schema: SIGNALS_SCHEMA.to_string(),
        artifact_id: bridge.packet.packet_id().to_string(),
        skill,
        topic: bridge.scenario.subject().to_string(),
        item,
        date: bridge.scenario.seed_label().to_string(),
        skill_version: provenance.producer_version().to_string(),
        input: provenance.input_id().to_string(),
        artifact_path: artifact.path().to_string(),
        extensions: bridge.extensions.clone(),
    })
}

#[test]
fn real_signals_sidecar_round_trips_losslessly() {
    let fixture = include_str!("fixtures/signals-artifact.v1.json");
    let sidecar: SignalsSidecar = serde_json::from_str(fixture).unwrap();
    let bridge = to_bridge(&sidecar).unwrap();
    let restored = from_bridge(&bridge).unwrap();

    assert_eq!(restored, sidecar);
    assert_eq!(
        bridge.extensions.keys().cloned().collect::<Vec<_>>(),
        ["experiment", "floor", "source_commit"]
    );
    assert_eq!(
        EvidencePacket::from_json(&bridge.packet.to_json().unwrap()).unwrap(),
        bridge.packet
    );
}

#[test]
fn unknown_signals_fields_remain_observable() {
    let fixture = include_str!("fixtures/signals-artifact.v1.json");
    let mut value: Value = serde_json::from_str(fixture).unwrap();
    value["future_context"] = Value::String("must survive".to_string());
    let sidecar: SignalsSidecar = serde_json::from_value(value).unwrap();
    let bridge = to_bridge(&sidecar).unwrap();

    assert_eq!(
        bridge.extensions.get("future_context"),
        Some(&Value::String("must survive".to_string()))
    );
    assert_eq!(from_bridge(&bridge).unwrap(), sidecar);
}

#[test]
fn unsupported_signals_schema_is_structured_failure() {
    let fixture = include_str!("fixtures/signals-artifact.v1.json");
    let mut sidecar: SignalsSidecar = serde_json::from_str(fixture).unwrap();
    sidecar.schema = "signals.artifact.v2".to_string();

    assert!(matches!(
        to_bridge(&sidecar),
        Err(Error::InvalidSchema { expected, actual })
            if expected == SIGNALS_SCHEMA && actual == "signals.artifact.v2"
    ));
}
