---
okf_version: "0.2"
type: Function
title: align_selected_to_grid_with_no_selection_stays_clean
description: "#426 — no selection is a clean no-op: no dirty flag, no undo"
resource: crates/oxide-app/src/library/editor/symbol/updates/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_no_selection_stays_clean
language: rust
---

# align_selected_to_grid_with_no_selection_stays_clean

#426 — no selection is a clean no-op: no dirty flag, no undo

## Signature

```rust
fn align_selected_to_grid_with_no_selection_stays_clean()
```

## Decorators

- `test`

## Docstring

#426 — no selection is a clean no-op: no dirty flag, no undo
snapshot. Mirrors the footprint editor's
`issue_146_align_to_grid_with_no_selection_stays_clean`.
[test]

## Source
Lines 147–158 in `crates/oxide-app/src/library/editor/symbol/updates/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/symbol/updates/transform.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/transform/new_editor.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
