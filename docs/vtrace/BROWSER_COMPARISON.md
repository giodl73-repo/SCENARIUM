# Browser comparison consumer — WP-WEB-001

Scope: a first-party, product-neutral consumer of existing run and comparison
contracts. No foundation requirement, adapter migration or adoption score is
reopened or claimed complete by this work.

| Contract | Consumer check | Evidence command |
| --- | --- | --- |
| Runs, comparison and identity (REQ-002–004,013–015) | Native decoder rejects forged identities; matching records compare deterministically | `cargo test --locked`; companion tests; `npm run test:pages` |
| Findings (REQ-005), caller provenance in run records | Error findings remain visible beside improved metrics; original extensions preserved; no packet integrity claim | Browser example, custody and native parity cases |
| Versioned portable interchange (REQ-007) | Native comparison JSON returned unchanged; all four outcomes and target direction | Native reference fixtures versus real WASM worker |
| Dependency and MSRV boundaries (REQ-011) | Foundation remains Rust 1.74; companion workspace unpublished and separate | Existing CI; locked native tests; package inspection |
| Consumer resource boundary | Bytes, depth, node and collection bounds; worker/file deadlines; stale events fenced | Companion tests and browser recovery cases |
| Inspectable delivery | Source archive, exact commit, dependency notices, WASM and per-file hashes | `python -B tools/build-pages.py`; release artifact verification |

Evidence is produced by this implementation's author and automated checks. It
is not an independent audit, human adoption approval, source authentication or
closure of previously exhausted foundation work packages. Existing
`REQUIREMENTS.md`, `TRACE.md` and their original verification records retain
their own scope and authority.
