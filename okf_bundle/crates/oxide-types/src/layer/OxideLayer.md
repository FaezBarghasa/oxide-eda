---
okf_version: "0.2"
type: Class
title: OxideLayer
description: "A PCB layer identified by purpose, not by index."
resource: crates/oxide-types/src/layer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/layer/OxideLayer
language: rust
---

# OxideLayer

A PCB layer identified by purpose, not by index.

## Signature

```rust
pub enum OxideLayer
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

A PCB layer identified by purpose, not by index.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(rename_all = "snake_case")]

## Source
Lines 23–44 in `crates/oxide-types/src/layer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layer](/crates/oxide-types/src/layer.md) |
