---
okf_version: "0.2"
type: Function
title: stale_status_message_clears_on_next_mutating_message
description: A stale status message (e.g. left over from a failed
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/stale_status_message_clears_on_next_mutating_message
language: rust
---

# stale_status_message_clears_on_next_mutating_message

A stale status message (e.g. left over from a failed

## Signature

```rust
fn stale_status_message_clears_on_next_mutating_message()
```

## Decorators

- `test`

## Docstring

A stale status message (e.g. left over from a failed
`JoinSelectionIntoPolygon`) is cleared by the very next
mutating message — here `DeleteSelected` on an empty
selection, itself a no-op — so it never lingers past the
action it described. Contract lives on
`SymbolEditorState::status_message`'s doc comment.
[test]

## Source
Lines 621–628 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
