---
okf_version: "0.2"
type: Function
title: cascade_personal_mode_auto_bumps_bound_row
description: Personal-mode cascade silently bumps a row binding to the saved
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/cascade_personal_mode_auto_bumps_bound_row
language: rust
---

# cascade_personal_mode_auto_bumps_bound_row

Personal-mode cascade silently bumps a row binding to the saved

## Signature

```rust
fn cascade_personal_mode_auto_bumps_bound_row()
```

## Decorators

- `test`

## Docstring

Personal-mode cascade silently bumps a row binding to the saved
symbol — both the row's pinned `symbol_version` and its own
`version` patch advance.
[test]

## Source
Lines 919–963 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [snx_manifest_with_mode](/crates/oxide-library/tests/local_git_adapter/snx_manifest_with_mode.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
