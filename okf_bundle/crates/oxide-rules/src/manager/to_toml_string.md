---
okf_version: "0.2"
type: Function
title: to_toml_string
description: Serialize constraints to a formatted TOML string.
resource: crates/oxide-rules/src/manager.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:29:13Z"
concept_id: crates/oxide-rules/src/manager/to_toml_string
language: rust
---

# to_toml_string

Serialize constraints to a formatted TOML string.

## Signature

```rust
impl ConstraintManager { pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> }
```

## Visibility

- `pub`

## Docstring

Serialize constraints to a formatted TOML string.

## Source
Lines 545–552 in `crates/oxide-rules/src/manager.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manager](/crates/oxide-rules/src/manager.md) |
| calls | [default_version](/crates/oxide-rules/src/manager/default_version.md) |
