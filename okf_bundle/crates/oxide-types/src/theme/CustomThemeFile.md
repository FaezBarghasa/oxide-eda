---
okf_version: "0.2"
type: Class
title: CustomThemeFile
description: A user-defined theme stored as JSON. Contains the full set of UI tokens
resource: crates/oxide-types/src/theme.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/theme/CustomThemeFile
language: rust
---

# CustomThemeFile

A user-defined theme stored as JSON. Contains the full set of UI tokens

## Signature

```rust
pub struct CustomThemeFile
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A user-defined theme stored as JSON. Contains the full set of UI tokens
and canvas palette colours so it can be round-tripped to/from disk.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `tokens`
- `canvas`

## Source
Lines 117–124 in `crates/oxide-types/src/theme.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme](/crates/oxide-types/src/theme.md) |
