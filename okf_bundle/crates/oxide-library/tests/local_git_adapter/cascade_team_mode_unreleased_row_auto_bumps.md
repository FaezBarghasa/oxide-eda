---
okf_version: "0.2"
type: Function
title: cascade_team_mode_unreleased_row_auto_bumps
description: Team-mode + unreleased row — auto-cascade fires (the released
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/cascade_team_mode_unreleased_row_auto_bumps
language: rust
---

# cascade_team_mode_unreleased_row_auto_bumps

Team-mode + unreleased row — auto-cascade fires (the released

## Signature

```rust
fn cascade_team_mode_unreleased_row_auto_bumps()
```

## Decorators

- `test`

## Docstring

Team-mode + unreleased row — auto-cascade fires (the released
flag is the gate, not the workflow mode alone). Mirrors Altium's
"edit-in-place is fine while the row is still in draft".
[test]

## Source
Lines 1032–1070 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [snx_manifest_with_mode](/crates/oxide-library/tests/local_git_adapter/snx_manifest_with_mode.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
