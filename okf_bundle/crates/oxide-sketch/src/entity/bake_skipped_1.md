---
okf_version: "0.2"
type: Function
title: bake_skipped
description: "v0.22 Phase A5 — `true` if the bake pipeline should skip this"
resource: crates/oxide-sketch/src/entity.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/entity/bake_skipped_1
language: rust
---

# bake_skipped

v0.22 Phase A5 — `true` if the bake pipeline should skip this

## Signature

```rust
pub fn bake_skipped(&self) -> bool
```

## Visibility

- `pub`

## Docstring

v0.22 Phase A5 — `true` if the bake pipeline should skip this
entity. Construction and Centerline both qualify. Used by every
`bake_*` site in `oxide-bake` to avoid lit-by-N copy of the
same `if entity.construction || entity.centerline` check.

## Source
Lines 114–116 in `crates/oxide-sketch/src/entity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entity](/crates/oxide-sketch/src/entity.md) |
