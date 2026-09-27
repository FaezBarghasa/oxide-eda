---
okf_version: "0.2"
type: Class
title: FeatureControlFrame
description: Feature Control Frame (FCF) per ASME Y14.5.
resource: crates/oxide-output/src/draftsman/gdt.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:19:20Z"
concept_id: crates/oxide-output/src/draftsman/gdt/FeatureControlFrame
language: rust
---

# FeatureControlFrame

Feature Control Frame (FCF) per ASME Y14.5.

## Signature

```rust
pub struct FeatureControlFrame
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Feature Control Frame (FCF) per ASME Y14.5.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `characteristic`
- `is_diameter_zone`
- `tolerance_value_mm`
- `material_condition`
- `primary_datum`
- `secondary_datum`
- `tertiary_datum`

## Source
Lines 52–60 in `crates/oxide-output/src/draftsman/gdt.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gdt](/crates/oxide-output/src/draftsman/gdt.md) |
