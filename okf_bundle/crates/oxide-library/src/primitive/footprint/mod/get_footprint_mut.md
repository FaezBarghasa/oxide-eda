---
okf_version: "0.2"
type: Function
title: get_footprint_mut
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/get_footprint_mut
language: rust
---

# get_footprint_mut

## Signature

```rust
impl FootprintFile { pub fn get_footprint_mut(&mut self, uuid: Uuid) -> Option<&mut Footprint> }
```

## Visibility

- `pub`

## Source
Lines 738–740 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
