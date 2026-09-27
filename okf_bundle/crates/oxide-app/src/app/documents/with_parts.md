---
okf_version: "0.2"
type: Function
title: with_parts
description: Closure-shaped split-borrow. The closure runs with both halves
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/with_parts
language: rust
---

# with_parts

Closure-shaped split-borrow. The closure runs with both halves

## Signature

```rust
impl FootprintEditorState { pub fn with_parts(&mut self, f: F) -> R }
```

## Type Parameters

- `R`
- `F`

## Visibility

- `pub`

## Docstring

Closure-shaped split-borrow. The closure runs with both halves
in scope; the borrows are scoped to its body. Equivalent to
[`parts_mut`](Self::parts_mut) but plays nicer when the call
site needs to touch other `editor` fields after the split-
borrowed work returns — the closure boundary forces NLL to
release the borrows promptly.

## Source
Lines 628–637 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
