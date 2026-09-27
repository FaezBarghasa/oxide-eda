---
okf_version: "0.2"
type: Class
title: SchematicProperty
description: Captures Standard property metadata without forcing all callers off the legacy
resource: crates/oxide-types/src/property.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/property/SchematicProperty
language: rust
---

# SchematicProperty

Captures Standard property metadata without forcing all callers off the legacy

## Signature

```rust
pub struct SchematicProperty
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Captures Standard property metadata without forcing all callers off the legacy
key/value field map at once.
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `key`
- `value`
- `id`
- `text`
- `show_name`
- `do_not_autoplace`
- `variant_overrides`

## Source
Lines 9–25 in `crates/oxide-types/src/property.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [property](/crates/oxide-types/src/property.md) |
