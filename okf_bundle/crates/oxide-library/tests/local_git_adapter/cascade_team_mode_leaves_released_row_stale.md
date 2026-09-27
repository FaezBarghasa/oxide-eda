---
okf_version: "0.2"
type: Function
title: cascade_team_mode_leaves_released_row_stale
description: Team-mode + released row — cascade leaves the pinned
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale
language: rust
---

# cascade_team_mode_leaves_released_row_stale

Team-mode + released row — cascade leaves the pinned

## Signature

```rust
fn cascade_team_mode_leaves_released_row_stale()
```

## Decorators

- `test`

## Docstring

Team-mode + released row — cascade leaves the pinned
`symbol_version` alone so the schematic doesn't auto-pull a
breaking change. The Library Browser's stale-binding indicator
catches the drift visually.
[test]

## Source
Lines 970–1026 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [snx_manifest_with_mode](/crates/oxide-library/tests/local_git_adapter/snx_manifest_with_mode.md) |
| calls | [fixture_symbol](/crates/oxide-library/tests/local_git_adapter/fixture_symbol.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
| calls | [cascade_after_symbol_save](/crates/oxide-library/src/cascade/cascade_after_symbol_save.md) |
