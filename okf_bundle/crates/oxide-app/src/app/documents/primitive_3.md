---
okf_version: "0.2"
type: Function
title: primitive
description: Borrow the footprint currently being edited. Falls back to
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/primitive_3
language: rust
---

# primitive

Borrow the footprint currently being edited. Falls back to

## Signature

```rust
pub fn primitive(&self) -> &Footprint
```

## Visibility

- `pub`

## Docstring

Borrow the footprint currently being edited. Falls back to
the first footprint when `active_idx` is out of range
(defensive — should never happen in practice). Panics only
when the file has zero footprints, which the loader rejects.

## Source
Lines 581–586 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
