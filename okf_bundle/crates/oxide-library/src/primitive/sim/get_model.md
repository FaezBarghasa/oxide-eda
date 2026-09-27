---
okf_version: "0.2"
type: Function
title: get_model
description: Locate a model by UUID within this file.
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/get_model
language: rust
---

# get_model

Locate a model by UUID within this file.

## Signature

```rust
impl SimFile { pub fn get_model(&self, uuid: Uuid) -> Option<&SimModel> }
```

## Visibility

- `pub`

## Docstring

Locate a model by UUID within this file.

## Source
Lines 252–254 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
