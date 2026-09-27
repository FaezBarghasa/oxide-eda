---
okf_version: "0.2"
type: Class
title: DatasheetRef
description: Reference to a datasheet — either a remote URL or a hash-pinned local PDF.
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/DatasheetRef
language: rust
---

# DatasheetRef

Reference to a datasheet — either a remote URL or a hash-pinned local PDF.

## Signature

```rust
pub enum DatasheetRef
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

Reference to a datasheet — either a remote URL or a hash-pinned local PDF.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "snake_case")]

## Methods

- `url`
- `hash`
- `filename`

## Source
Lines 27–30 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |
