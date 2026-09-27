---
okf_version: "0.2"
type: Function
title: strip_eq_prefix
description: "Strip the optional Altium-style leading `=` and surrounding"
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/strip_eq_prefix
language: rust
---

# strip_eq_prefix

Strip the optional Altium-style leading `=` and surrounding

## Signature

```rust
fn strip_eq_prefix(src: &str) -> &str
```

## Docstring

Strip the optional Altium-style leading `=` and surrounding
whitespace from a parameter source string.

## Source
Lines 65–68 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| called_by | [resolve](/crates/oxide-sketch/src/parameter/resolve.md) |
