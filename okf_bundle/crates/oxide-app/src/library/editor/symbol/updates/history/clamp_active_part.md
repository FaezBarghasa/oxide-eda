---
okf_version: "0.2"
type: Function
title: clamp_active_part
description: "Re-clamp the editor's `active_part` into `1..=max_part_number`"
resource: crates/oxide-app/src/library/editor/symbol/updates/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/history/clamp_active_part
language: rust
---

# clamp_active_part

Re-clamp the editor's `active_part` into `1..=max_part_number`

## Signature

```rust
fn clamp_active_part(editor: &mut SymEditor)
```

## Docstring

Re-clamp the editor's `active_part` into `1..=max_part_number`
after a snapshot restore. `active_part` lives on the editor state,
not inside the `Symbol` snapshot, so undo/redo can otherwise leave
it pointing past the restored unit count (e.g. undoing a New Part).

## Source
Lines 43–46 in `crates/oxide-app/src/library/editor/symbol/updates/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/library/editor/symbol/updates/history.md) |
| calls | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
| called_by | [apply_symbol_history](/crates/oxide-app/src/library/editor/symbol/updates/history/apply_symbol_history.md) |
