---
name: scenarium-pulse
description: Execute the next SCENARIUM wave pulse with implementation, validation, and commit-ready docs.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# SCENARIUM Pulse

1. Read `context/waves/PHASES.md`.
2. Read the active `WAVE.md` and target pulse.
3. Implement the smallest complete evidence-contract slice.
4. Preserve consumer ownership and structured failure behavior.
5. Update docs and pulse status.
6. Run fmt, clippy, tests, and `git diff --check`.

