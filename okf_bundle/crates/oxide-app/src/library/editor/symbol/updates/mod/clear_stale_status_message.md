---
okf_version: "0.2"
type: Function
title: clear_stale_status_message
description: "`SymbolEditorState::status_message`'s contract is \"cleared on the"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/clear_stale_status_message
language: rust
---

# clear_stale_status_message

`SymbolEditorState::status_message`'s contract is "cleared on the

## Signature

```rust
fn clear_stale_status_message(editor: &mut SymEditor, msg: &SymbolEditorMsg)
```

## Docstring

`SymbolEditorState::status_message`'s contract is "cleared on the
next successful action" — enforced centrally here, once, rather
than at every individual mutation's success point. Clears for
every message that represents an attempted document/selection
mutation; left untouched for the continuous or chrome-only
messages that fire on every frame/hover and would otherwise flash
a just-set message away before the user can read it (camera pan/
zoom/cursor readout, and the context-menu / active-bar-menu
open/close/toggle chrome, which manage their own state and aren't
"actions" against the symbol itself). A message that itself fails
(e.g. a second bad `JoinSelectionIntoPolygon`) re-sets the message
right after this clear, so the net effect always reflects the
outcome of the most recent relevant action.

## Source
Lines 90–108 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
