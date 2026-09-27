---
okf_version: "0.2"
type: Class
title: ParameterTable
description: "User-defined parameter table — `name → source-string`. Source"
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/ParameterTable
language: rust
---

# ParameterTable

User-defined parameter table — `name → source-string`. Source

## Signature

```rust
pub struct ParameterTable
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

User-defined parameter table — `name → source-string`. Source
strings carry an optional `=` prefix (Altium-style) and are
otherwise the same expression-language input that the parser
accepts.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 34–34 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
