---
okf_version: "0.2"
type: Function
title: push_undo_snapshot
description: Push a pre-captured snapshot onto the undo stack and clear the redo
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo_snapshot
language: rust
---

# push_undo_snapshot

Push a pre-captured snapshot onto the undo stack and clear the redo

## Signature

```rust
fn push_undo_snapshot(editor: &mut SymEditor, snapshot: oxide_library::Symbol)
```

## Docstring

Push a pre-captured snapshot onto the undo stack and clear the redo
stack. Split out of [`push_undo`] so a caller that must know
*whether* a mutation actually happened before committing to the undo
push (e.g. a snap that can be a no-op) can clone the pre-mutation
state up front, perform the mutation, and only call this — snapshot
push and redo-clear together, as one unit — once it has confirmed
something changed. Doing the two independently (push now, maybe pop
the snapshot back off later) still leaves the redo stack cleared on
a no-op, which is the bug this split exists to prevent.

## Source
Lines 54–60 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
