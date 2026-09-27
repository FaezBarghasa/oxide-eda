---
okf_version: "0.2"
type: Class
title: ParamValue
description: One parameter cell. String / number / bool / measurement (value + unit).
resource: crates/oxide-library/src/param.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/param/ParamValue
language: rust
---

# ParamValue

One parameter cell. String / number / bool / measurement (value + unit).

## Signature

```rust
pub enum ParamValue
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", content = "value")`

## Visibility

- `pub`

## Docstring

One parameter cell. String / number / bool / measurement (value + unit).
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", content = "value")]

## Methods

- `value`
- `unit`

## Source
Lines 12–17 in `crates/oxide-library/src/param.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param](/crates/oxide-library/src/param.md) |
