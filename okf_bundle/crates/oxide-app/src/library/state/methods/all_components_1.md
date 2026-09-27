---
okf_version: "0.2"
type: Function
title: all_components
description: "Aggregate every open library's cached components — used by the"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/all_components_1
language: rust
---

# all_components

Aggregate every open library's cached components — used by the

## Signature

```rust
pub fn all_components(&self) -> Vec<(PathBuf, ComponentSummary)>
```

## Visibility

- `pub`

## Docstring

Aggregate every open library's cached components — used by the
picker modal to flatten across libraries.

## Source
Lines 268–276 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
