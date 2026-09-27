---
okf_version: "0.2"
type: Function
title: get_footprint
description: Locate a footprint by UUID within this file.
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/get_footprint
language: rust
---

# get_footprint

Locate a footprint by UUID within this file.

## Signature

```rust
impl FootprintFile { pub fn get_footprint(&self, uuid: Uuid) -> Option<&Footprint> }
```

## Visibility

- `pub`

## Docstring

Locate a footprint by UUID within this file.

## Source
Lines 734–736 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
