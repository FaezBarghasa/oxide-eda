---
okf_version: "0.2"
type: Function
title: find_model_for_symbol
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod/find_model_for_symbol
language: rust
---

# find_model_for_symbol

## Signature

```rust
impl PSpiceDeckBuilder<'a> { fn find_model_for_symbol(&self, _refdes: &str, value: &str, lib_id: &str) -> Option<&SimModel> }
```

## Type Parameters

- `'a`

## Source
Lines 272–280 in `crates/oxide-sim/src/deck/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [deck](/crates/oxide-sim/src/deck/mod.md) |
