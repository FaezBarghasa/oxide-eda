---
okf_version: "0.2"
type: Function
title: from_toml_str
description: Parse constraints from a TOML configuration string.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/from_toml_str_1
language: rust
---

# from_toml_str

Parse constraints from a TOML configuration string.

## Signature

```rust
pub fn from_toml_str(toml_content: &str) -> Result<Self, toml::de::Error>
```

## Visibility

- `pub`

## Docstring

Parse constraints from a TOML configuration string.

## Source
Lines 537–542 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
