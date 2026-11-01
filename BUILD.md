# Reproduce the comparison workbench

The browser calls the existing SCENARIUM `RunRecord::from_json`,
`compare_runs`, and `ComparisonReport::to_json` through the unpublished
`web/adapter` companion. The foundation manifest, lockfile, API and Rust 1.74
support are unchanged. The companion is a separate workspace using Rust 1.95.0
and wasm-bindgen 0.2.127; its only feature is `wasm`.

Install Rust 1.95.0 with rustfmt, Clippy and wasm32-unknown-unknown, Python 3.12,
Node 22, and wasm-bindgen-cli 0.2.127. Then run:

```sh
cargo test --locked
cargo fmt --manifest-path web/adapter/Cargo.toml --check
cargo test --locked --manifest-path web/adapter/Cargo.toml --all-features
cargo clippy --locked --manifest-path web/adapter/Cargo.toml --all-targets --all-features -- -D warnings
npm ci
npx playwright install chromium
python -B tools/build-pages.py
npm run test:pages
python -B tools/serve-pages.py
```

Open http://127.0.0.1:4177/SCENARIUM/. The Pages workflow pins the companion
compiler, bindings archive checksum, Node version and npm lockfile. The existing
CI separately tests the foundation on stable and Rust 1.74.0. A clean build
embeds its exact commit in `provenance.json` and `source.zip`; a dirty preview
is explicitly marked and includes staged source files. Preview artifacts are
not release evidence. `.cache/site-inventory.json` hashes every delivered file.
Dependency notices are collected from the resolved adapter closure, including
proc macros. Rust standard library notices accompany the compiled artifact.

## Input and interpretation

Supply two versioned SCENARIUM run records, using the examples as a starting
point. The core checks identities, roles, scenario compatibility, metric sets
and directions. Metric outcomes are descriptive. An improved outcome can coexist
with error findings. Findings and caller-asserted provenance remain visible;
this tool neither runs experiments nor authenticates their source or recommends
adoption. Consumers own metric meaning and policy.

The companion bounds each record to 1 MiB UTF-8, its request envelope to 3 MiB,
JSON container depth to 32, JSON nodes to 50,000 per record, and metrics and
findings to 256 each. These are input bounds, not a browser heap quota. A worker
has a 12-second startup and 10-second comparison deadline. Local reads have a
five-second deadline. Failed or stale work retains the previous draft.

Original input downloads preserve valid uploaded UTF-8 text, including CRLF,
until an edit, successful replacement, example selection or reset. Unknown
fields ignored by native decoding are listed as JSON pointers. Known-field
views omit them and are labelled accordingly. Comparison downloads retain the
native JSON string; display numbers are rounded to eight significant digits.
Browser JavaScript never parses and serializes an original record for export,
so extension integers beyond JavaScript's exact range remain intact.

## Evidence

The builder creates native golden responses for all examples and rejection
fixtures. Browser tests load the real WASM worker, compare its exact response
strings against those references, test file custody and recovery, and capture
desktop/mobile results. `browser-evidence` and `github-pages` are separate
workflow artifacts. Deploy runs only after browser checks pass on main.

This is a first-party consumer of existing contracts. It adds no shared
abstraction, external adopter, migration/deletion score or registry publication.
