---
okf_version: "0.2"
type: Function
title: from_toml_str
description: Parse the TOML wire format. Format-token mismatch surfaces
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/from_toml_str
language: rust
---

# from_toml_str

Parse the TOML wire format. Format-token mismatch surfaces

## Signature

```rust
impl SimFile { pub fn from_toml_str(text: &str) -> Result<Self, SimFileError> }
```

## Visibility

- `pub`

## Docstring

Parse the TOML wire format. Format-token mismatch surfaces
[`SimFileError::UnsupportedFormat`].

## Source
Lines 166–194 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
