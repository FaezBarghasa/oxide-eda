---
okf_version: "0.2"
type: Class
title: LibraryMode
description: "[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/LibraryMode
language: rust
---

# LibraryMode

[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub enum LibraryMode
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "snake_case")]

## Methods

- `url`
- `auth`

## Source
Lines 36–43 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |
