---
okf_version: "0.2"
type: Function
title: apply_symbol_history
resource: crates/oxide-app/src/library/editor/symbol/updates/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/history/apply_symbol_history
language: rust
---

# apply_symbol_history

## Signature

```rust
pub(super) fn apply_symbol_history(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 6–37 in `crates/oxide-app/src/library/editor/symbol/updates/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/library/editor/symbol/updates/history.md) |
| calls | [close_pickers](/crates/oxide-app/src/library/editor/symbol/updates/mod/close_pickers.md) |
| calls | [clamp_active_part](/crates/oxide-app/src/library/editor/symbol/updates/history/clamp_active_part.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
