//! Workbench policy; no foundation schema, metrics or decision policy changes.
use scenarium::{compare_runs, RunRecord};
use serde::Deserialize;
use serde_json::{json, Value};
pub const MAX_RUN: usize = 1024 * 1024;
pub const MAX_REQUEST: usize = 3 * MAX_RUN;
const SCHEMA: &str = "scenarium.comparison-workbench.v1";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    baseline_json: String,
    candidate_json: String,
}
fn failure(phase: &str, message: &str) -> Value {
    json!({"schema":SCHEMA,"ok":false,"phase":phase,"diagnostic":{"message":message}})
}
fn bounded(value: &Value, nodes: &mut usize) -> bool {
    *nodes += 1;
    if *nodes > 50_000 {
        return false;
    }
    match value {
        Value::Array(v) => v.iter().all(|x| bounded(x, nodes)),
        Value::Object(v) => v.values().all(|x| bounded(x, nodes)),
        _ => true,
    }
}
fn preflight(text: &str, phase: &str) -> Result<Value, Value> {
    if text.len() > MAX_RUN {
        return Err(failure(
            phase,
            "Run exceeds the workbench's 1 MiB UTF-8 limit.",
        ));
    }
    let (mut quoted, mut escaped, mut depth) = (false, false, 0usize);
    for b in text.bytes() {
        if quoted {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                quoted = false;
            }
        } else {
            match b {
                b'"' => quoted = true,
                b'{' | b'[' => {
                    depth += 1;
                    if depth > 32 {
                        return Err(failure(phase, "JSON nesting exceeds 32 containers."));
                    }
                }
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => (),
            }
        }
    }
    let v:Value=serde_json::from_str(text).map_err(|e|json!({"schema":SCHEMA,"ok":false,"phase":phase,"diagnostic":{"message":e.to_string(),"line":e.line(),"column":e.column()}}))?;
    if !bounded(&v, &mut 0) {
        return Err(failure(phase, "Run exceeds 50,000 JSON nodes."));
    }
    if v.pointer("/payload/metrics")
        .and_then(Value::as_object)
        .is_some_and(|m| m.len() > 256)
    {
        return Err(failure(
            phase,
            "Workbench accepts at most 256 metrics per run.",
        ));
    }
    if v.pointer("/payload/findings")
        .and_then(Value::as_array)
        .is_some_and(|m| m.len() > 256)
    {
        return Err(failure(
            phase,
            "Workbench accepts at most 256 findings per run.",
        ));
    }
    Ok(v)
}
fn unknown(raw: &Value, known: &Value, path: &str, output: &mut Vec<String>) {
    match (raw, known) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in a {
                let key = format!("{path}/{}", k.replace('~', "~0").replace('/', "~1"));
                if let Some(w) = b.get(k) {
                    unknown(v, w, &key, output);
                } else {
                    output.push(key);
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            for (i, (v, w)) in a.iter().zip(b).enumerate() {
                unknown(v, w, &format!("{path}/{i}"), output);
            }
        }
        _ => (),
    }
}
fn response(input: &str) -> Value {
    if input.len() > MAX_REQUEST {
        return failure("request", "Envelope exceeds 3 MiB UTF-8 bytes.");
    }
    let req: Request = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => return failure("request", &e.to_string()),
    };
    let mut decoded = Vec::new();
    for (phase, text) in [
        ("baseline", &req.baseline_json),
        ("candidate", &req.candidate_json),
    ] {
        let raw = match preflight(text, phase) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let run = match RunRecord::from_json(text) {
            Ok(v) => v,
            Err(e) => return failure(phase, &e.to_string()),
        };
        let normalized = match run.to_json() {
            Ok(v) => v,
            Err(e) => return failure(phase, &e.to_string()),
        };
        let normalized_value: Value = serde_json::from_str(&normalized).expect("native JSON");
        let mut extras = Vec::new();
        unknown(&raw, &normalized_value, "", &mut extras);
        decoded.push((run, normalized, extras));
    }
    let report = match compare_runs(&decoded[0].0, &decoded[1].0) {
        Ok(v) => v,
        Err(e) => return failure("comparison", &e.to_string()),
    };
    let report_json = match report.to_json() {
        Ok(v) => v,
        Err(e) => return failure("comparison", &e.to_string()),
    };
    json!({"schema":SCHEMA,"ok":true,"outcome":report.status().as_str(),"comparison_json":report_json,
        "baseline":{"known_json":decoded[0].1,"unknown_fields":decoded[0].2,"run_status":decoded[0].0.status()},
        "candidate":{"known_json":decoded[1].1,"unknown_fields":decoded[1].2,"run_status":decoded[1].0.status()}})
}
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn compare(input: &str) -> String {
    response(input).to_string()
}

pub fn examples() -> Value {
    use scenarium::{
        Finding, Metric, MetricDirection as D, Provenance, RunVariant, Scenario, Severity,
    };
    let mut result = serde_json::Map::new();
    for (name, completion, latency) in [
        ("mixed", 0.83, 130.0),
        ("improved", 0.83, 80.0),
        ("regressed", 0.40, 130.0),
        ("equivalent", 0.61, 100.0),
        ("target", 0.83, 80.0),
        ("error-findings", 0.83, 80.0),
    ] {
        let scenario = Scenario::new(
            "service-fixture",
            "Synthetic caller-supplied run comparison",
            "seed-1",
        )
        .unwrap();
        let mut baseline =
            RunRecord::new(&scenario, RunVariant::Inertia, "browser-example").unwrap();
        let mut candidate = RunRecord::new(
            &scenario,
            RunVariant::candidate("candidate-v1").unwrap(),
            "browser-example",
        )
        .unwrap();
        baseline
            .record_metric(Metric::new("completion_rate", 0.61, D::HigherIsBetter).unwrap())
            .unwrap();
        candidate
            .record_metric(Metric::new("completion_rate", completion, D::HigherIsBetter).unwrap())
            .unwrap();
        let direction = if name == "target" {
            D::Target(90.0)
        } else {
            D::LowerIsBetter
        };
        baseline
            .record_metric(Metric::new("latency_ms", 100.0, direction.clone()).unwrap())
            .unwrap();
        candidate
            .record_metric(Metric::new("latency_ms", latency, direction).unwrap())
            .unwrap();
        for run in [&mut baseline, &mut candidate] {
            run.set_provenance(
                Provenance::new("synthetic-workbench", "1", "authored-example").unwrap(),
            );
        }
        if name == "error-findings" {
            candidate.add_finding(
                Finding::new(
                    Severity::Error,
                    "example-error",
                    "fixture",
                    "Synthetic error finding: better metrics do not erase this finding.",
                )
                .unwrap(),
            );
        }
        result.insert(name.into(),json!({"baseline_json":baseline.to_json().unwrap(),"candidate_json":candidate.to_json().unwrap()}));
    }
    Value::Object(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_outcomes_and_error_findings_remain_distinct() {
        for name in ["mixed", "improved", "regressed", "equivalent"] {
            assert_eq!(response(&examples()[name].to_string())["outcome"], name);
        }
        let r = response(&examples()["error-findings"].to_string());
        assert_eq!(r["outcome"], "improved");
        assert_eq!(r["candidate"]["run_status"], "error");
    }
    #[test]
    fn native_reports_roundtrip_and_repeat() {
        for (_, example) in examples().as_object().unwrap() {
            let req = example.to_string();
            let r = response(&req);
            let report =
                scenarium::ComparisonReport::from_json(r["comparison_json"].as_str().unwrap())
                    .unwrap();
            assert_eq!(report.status().as_str(), r["outcome"]);
            assert_eq!(compare(&req), compare(&req));
        }
    }
    #[test]
    fn unknown_extensions_are_reported_without_claiming_preservation_in_known_view() {
        let mut req = examples()["mixed"].clone();
        let mut run: Value = serde_json::from_str(req["candidate_json"].as_str().unwrap()).unwrap();
        run["payload"]["extra"] = json!({"large_integer":18446744073709551615u64});
        req["candidate_json"] = json!(run.to_string());
        let r = response(&req.to_string());
        assert_eq!(r["ok"], true);
        assert_eq!(r["candidate"]["unknown_fields"], json!(["/payload/extra"]));
        assert!(!r["candidate"]["known_json"]
            .as_str()
            .unwrap()
            .contains("large_integer"));
    }
    #[test]
    fn invalid_schema_identity_metric_sets_and_types_fail() {
        assert_eq!(response("{}")["ok"], false);
        for change in ["schema", "identity", "metrics"] {
            let mut req = examples()["mixed"].clone();
            let mut run: Value =
                serde_json::from_str(req["candidate_json"].as_str().unwrap()).unwrap();
            match change {
                "schema" => run["schema"] = json!("scenarium.v2"),
                "identity" => run["payload"]["run_id"] = json!("forged"),
                _ => {
                    run["payload"]["metrics"]
                        .as_object_mut()
                        .unwrap()
                        .remove("latency_ms");
                }
            }
            req["candidate_json"] = json!(run.to_string());
            assert_eq!(response(&req.to_string())["ok"], false);
        }
    }
    #[test]
    fn resource_limits_precede_native_parse() {
        assert_eq!(
            preflight(&" ".repeat(MAX_RUN + 1), "candidate").unwrap_err()["ok"],
            false
        );
        assert_eq!(
            preflight(
                &format!("{}0{}", "[".repeat(33), "]".repeat(33)),
                "candidate"
            )
            .unwrap_err()["ok"],
            false
        );
        assert_eq!(response(&" ".repeat(MAX_REQUEST + 1))["phase"], "request");
    }

    #[test]
    fn node_and_collection_limits_accept_boundary_and_reject_overflow() {
        assert!(preflight(&json!(vec![0; 49_999]).to_string(), "baseline").is_ok());
        assert!(preflight(&json!(vec![0; 50_000]).to_string(), "baseline").is_err());
        for field in ["metrics", "findings"] {
            for count in [256, 257] {
                let mut doc = json!({"payload":{}});
                doc["payload"][field] = if field == "metrics" {
                    Value::Object((0..count).map(|i| (i.to_string(), json!(0))).collect())
                } else {
                    json!(vec![0; count])
                };
                assert_eq!(
                    preflight(&doc.to_string(), "baseline").is_ok(),
                    count == 256
                );
            }
        }
        assert!(preflight(
            &format!("{}0{}", "[".repeat(32), "]".repeat(32)),
            "baseline"
        )
        .is_ok());
    }
}
