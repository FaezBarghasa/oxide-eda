---
okf_version: "0.2"
type: Class
title: PasteAperturePattern
description: Solder-paste aperture layout for a pad.
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/PasteAperturePattern
language: rust
---

# PasteAperturePattern

Solder-paste aperture layout for a pad.

## Signature

```rust
pub enum PasteAperturePattern
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`
- `derive(Default)`

## Visibility

- `pub`

## Docstring

Solder-paste aperture layout for a pad.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]
[derive(Default)]

## Methods

- `nx_expr`
- `ny_expr`
- `coverage_expr`
- `source`

## Source
Lines 390–401 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
