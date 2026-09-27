---
okf_version: "0.2"
type: Function
title: primitive
description: Borrow the symbol currently being edited. Falls back to the
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/primitive_1
language: rust
---

# primitive

Borrow the symbol currently being edited. Falls back to the

## Signature

```rust
pub fn primitive(&self) -> &Symbol
```

## Visibility

- `pub`

## Docstring

Borrow the symbol currently being edited. Falls back to the
first symbol in the file when `active_idx` is out of range
(defensive — should never happen in practice). Panics only
when the file has zero symbols, which the loader rejects.

## Source
Lines 373–378 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
