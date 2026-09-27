---
okf_version: "0.2"
type: Function
title: table_for_class
description: Resolve a class name to its table filename stem.
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/table_for_class
language: rust
---

# table_for_class

Resolve a class name to its table filename stem.

## Signature

```rust
impl Manifest { pub fn table_for_class(&self, class: &str) -> String }
```

## Visibility

- `pub`

## Docstring

Resolve a class name to its table filename stem.

1. If any `[[tables]]` override lists `class` in its `classes` array,
return that override's `name`.
2. Otherwise default-pluralise: `"resistor"` → `"resistors"`.
The plural is mechanical (`s` suffix); irregulars need an explicit
override.

## Source
Lines 161–168 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |
