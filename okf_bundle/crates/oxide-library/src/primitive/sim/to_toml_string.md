---
okf_version: "0.2"
type: Function
title: to_toml_string
description: "Serialise to canonical TOML. The per-model `body` field is"
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/to_toml_string
language: rust
---

# to_toml_string

Serialise to canonical TOML. The per-model `body` field is

## Signature

```rust
impl SimFile { pub fn to_toml_string(&self) -> Result<String, SimFileError> }
```

## Visibility

- `pub`

## Docstring

Serialise to canonical TOML. The per-model `body` field is
emitted as a `body = '''…'''` literal multi-line string via a
sentinel-replace pass post-`to_string_pretty`.

## Source
Lines 199–249 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
