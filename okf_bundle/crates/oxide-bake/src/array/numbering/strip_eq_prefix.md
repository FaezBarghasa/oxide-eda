---
okf_version: "0.2"
type: Function
title: strip_eq_prefix
description: "Strip the optional Altium-style leading `=` and surrounding"
resource: crates/oxide-bake/src/array/numbering.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/array/numbering/strip_eq_prefix
language: rust
---

# strip_eq_prefix

Strip the optional Altium-style leading `=` and surrounding

## Signature

```rust
pub(super) fn strip_eq_prefix(src: &str) -> &str
```

## Visibility

- `pub(super)`

## Docstring

Strip the optional Altium-style leading `=` and surrounding
whitespace so authored expressions like `= count` parse cleanly.

## Source
Lines 116–119 in `crates/oxide-bake/src/array/numbering.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [numbering](/crates/oxide-bake/src/array/numbering.md) |
| called_by | [eval_numbering_expr](/crates/oxide-bake/src/array/numbering/eval_numbering_expr.md) |
