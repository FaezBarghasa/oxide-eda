---
okf_version: "0.2"
type: Function
title: undo
description: "v0.24 Phase 1 — undo the most recent edit. Returns `true` if"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/undo
language: rust
---

# undo

v0.24 Phase 1 — undo the most recent edit. Returns `true` if

## Signature

```rust
impl FootprintEditorState { pub fn undo(&mut self) -> bool }
```

## Visibility

- `pub`

## Docstring

v0.24 Phase 1 — undo the most recent edit. Returns `true` if
a snapshot was popped + applied, `false` if the history was
empty (i.e. nothing to undo). The current state is pushed
onto the redo stack so `redo()` can walk forward again.

## Source
Lines 511–522 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
