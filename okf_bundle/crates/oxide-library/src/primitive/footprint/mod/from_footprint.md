---
okf_version: "0.2"
type: Function
title: from_footprint
description: Build a new container holding a single footprint — what the
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/from_footprint
language: rust
---

# from_footprint

Build a new container holding a single footprint — what the

## Signature

```rust
impl FootprintFile { pub fn from_footprint(footprint: Footprint) -> Self }
```

## Visibility

- `pub`

## Docstring

Build a new container holding a single footprint — what the
`Add New ▸ Footprint Library` flow seeds.

## Source
Lines 602–612 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
| calls | [default_footprint_format](/crates/oxide-library/src/primitive/footprint/mod/default_footprint_format.md) |
