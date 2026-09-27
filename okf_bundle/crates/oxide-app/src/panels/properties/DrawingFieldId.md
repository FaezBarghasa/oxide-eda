---
okf_version: "0.2"
type: Class
title: DrawingFieldId
description: Stable identifiers for every numeric drawing-field editor so the
resource: crates/oxide-app/src/panels/properties.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/properties/DrawingFieldId
language: rust
---

# DrawingFieldId

Stable identifiers for every numeric drawing-field editor so the

## Signature

```rust
pub enum DrawingFieldId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Stable identifiers for every numeric drawing-field editor so the
panel keeps a transient string buffer per field across rerenders.
Erasing a text_input leaves an empty string in the buffer until
the user types a valid f64, at which point UpdateDrawingEdit fires.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 46–68 in `crates/oxide-app/src/panels/properties.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [properties](/crates/oxide-app/src/panels/properties.md) |
