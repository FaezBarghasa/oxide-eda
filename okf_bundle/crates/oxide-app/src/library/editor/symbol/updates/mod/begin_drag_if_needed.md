---
okf_version: "0.2"
type: Function
title: begin_drag_if_needed
description: Record the first event of a drag gesture.
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/begin_drag_if_needed
language: rust
---

# begin_drag_if_needed

Record the first event of a drag gesture.

## Signature

```rust
fn begin_drag_if_needed(editor: &mut SymEditor)
```

## Docstring

Record the first event of a drag gesture.
Subsequent events in the same drag are no-ops (mid_drag stays true).

## Source
Lines 64–69 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| called_by | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
