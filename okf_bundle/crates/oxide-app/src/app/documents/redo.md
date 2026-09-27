---
okf_version: "0.2"
type: Function
title: redo
description: v0.24 Phase 1 — redo the most recently undone edit. Returns
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/redo
language: rust
---

# redo

v0.24 Phase 1 — redo the most recently undone edit. Returns

## Signature

```rust
impl FootprintEditorState { pub fn redo(&mut self) -> bool }
```

## Visibility

- `pub`

## Docstring

v0.24 Phase 1 — redo the most recently undone edit. Returns
`true` if a redo snapshot was popped + applied, `false` when
the redo stack is empty.

## Source
Lines 527–538 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
