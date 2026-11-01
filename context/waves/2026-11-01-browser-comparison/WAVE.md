# Browser run comparison

Entry: public main be7cb4726c83c3162c660bf251bd56d4e642230d.
Goal: load two caller-supplied scenarium.v1 RunRecord envelopes, recompute their
comparison using existing Rust, inspect metric directions/deltas, findings-derived
run statuses and provenance, and download the native comparison/report plus
unchanged input text. Start with native-generated synthetic examples covering
improved, regressed, mixed, equivalent, target direction and error findings.

Named capability/adopter: the repository's first-party browser workbench consumes
existing SCENARIUM APIs. `web/adapter` is a standalone unpublished consumer
workspace. Foundation source, normal dependencies, features, lock and Rust 1.74
contract remain unchanged. No neutral-type generalization, consumer migration,
deletion/adoption gain, registry publication, execution engine or decision policy.
No claim of experiment execution, statistical significance, input authenticity,
packet closure or adoption recommendation. Native comparison statuses and error
findings remain visible together even when metrics improve.

| Pulse | Deliverable | Status |
|---|---|---|
| 01 | Bounded native/worker adapter, own files/inputs, source/download/recovery UI and Pages proof | implemented; release qualification required |

Bounds: 1 MiB per run, 3 MiB request, JSON depth 32 / 50,000 nodes per run,
256 metrics and 256 findings per run. Limits belong to the workbench, not v1.
Post-parse counts are not allocation quotas. Preserve original opened UTF-8 bytes
and CRLF until an edit/successful replacement/Restore example. Unknown fields
ignored by the native parser are reported; originals remain downloadable and
normalized known-field views are explicitly labelled.

Validate foundation fmt/Clippy/tests/docs/package and Rust 1.74 lane; new adapter
fmt/strict host/WASM Clippy, native examples/parity cases and real WASM browser
own-input/file/download/error/recovery/stale-event, keyboard/mobile/no-JS routes.
Clean pinned builds deploy only the tested main artifact; verify all live files
and replay browser cases before the TRACKER snapshot. Producer roles follow the
repository's stated order. The browser VTRACE supplement maps existing contract
requirements without promoting consumer adoption or foundation readiness scores.

The recorded TRACKER entry 863bf838626121c860e6a043a282fca9f236d7cd contained
earlier PITFALL governance work absent from public main. That complete branch
is merged alongside the companion, preserving its four policy checks and all
governance records. Foundation production source, manifest and lock remain
identical to both entries.

Local qualification: foundation's 18 tests pass on Rust 1.74.0 and 1.95.0;
foundation fmt, strict Clippy, docs and package verification pass. The companion's
six native tests cover outcome/report contracts, unknown fields, invalid records
and admission limits. Strict host and WASM Clippy pass. The pinned build generates
21 native reference responses and a 34-file site within the 5 MB delivery budget.
Nineteen browser checks exercise the real Rust worker, own inputs and all examples,
original UTF-8/CRLF/large-integer custody, invalid replacement, late file reads,
file/worker deadlines, keyboard restore, no-JS downloads and desktop/mobile layouts.
Source and compiled artifact remain independently inspectable. TRACKER owns the
subsequent exact-main hosted/live custody record; this source note is not a
claim that deployment has already completed.
