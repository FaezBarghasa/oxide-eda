---
okf_version: "0.2"
type: Class
title: ComponentClassEntry
description: "One entry in the user's component-class list. Persisted as a JSON"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/ComponentClassEntry
language: rust
---

# ComponentClassEntry

One entry in the user's component-class list. Persisted as a JSON

## Signature

```rust
pub struct ComponentClassEntry
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

One entry in the user's component-class list. Persisted as a JSON
object `{ "key": "...", "label": "..." }` inside the
`component_classes` array in `prefs.json`. `key` is the canonical
machine identifier stored on `ComponentRow.class`; `label` is the
human-readable name surfaced in pickers.
[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]

## Methods

- `key`
- `label`

## Source
Lines 107–110 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
