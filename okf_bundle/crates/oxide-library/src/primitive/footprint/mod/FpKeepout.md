---
okf_version: "0.2"
type: Class
title: FpKeepout
description: DRC keepout zone. v0.14 stores the polygon + layer + forbid kind.
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/FpKeepout
language: rust
---

# FpKeepout

DRC keepout zone. v0.14 stores the polygon + layer + forbid kind.

## Signature

```rust
pub struct FpKeepout
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

DRC keepout zone. v0.14 stores the polygon + layer + forbid kind.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `boundary`
- `layer`
- `forbids`

## Source
Lines 211–216 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
