---
okf_version: "0.2"
type: Class
title: NumberingSchemeKindUi
description: v0.23 — Numbering scheme kind for the Properties panel pick_list.
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/NumberingSchemeKindUi
language: rust
---

# NumberingSchemeKindUi

v0.23 — Numbering scheme kind for the Properties panel pick_list.

## Signature

```rust
pub enum NumberingSchemeKindUi
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.23 — Numbering scheme kind for the Properties panel pick_list.
Mirrors [`oxide_sketch::array::NumberingScheme`]'s tag. The handler
preserves the inner expression fields when flipping kinds where
possible (e.g. switching to LinearIncrement keeps any prior
start/step expressions; switching to Explicit clears them).
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 341–345 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
