---
okf_version: "0.2"
type: Function
title: primitive_mut
description: Mutable borrow of the active symbol — used by canvas mutations
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/primitive_mut
language: rust
---

# primitive_mut

Mutable borrow of the active symbol — used by canvas mutations

## Signature

```rust
impl SymbolEditorState { pub fn primitive_mut(&mut self) -> &mut Symbol }
```

## Visibility

- `pub`

## Docstring

Mutable borrow of the active symbol — used by canvas mutations
(add/move/delete pin etc.). Same out-of-range fallback as
[`primitive`](Self::primitive).

## Source
Lines 383–388 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
