---
okf_version: "0.2"
type: Class
title: PadShapeParamSummary
description: v0.24 Phase 3 (Track A2) — surface entry for one parametric pad
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/PadShapeParamSummary
language: rust
---

# PadShapeParamSummary

v0.24 Phase 3 (Track A2) — surface entry for one parametric pad

## Signature

```rust
pub struct PadShapeParamSummary
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.24 Phase 3 (Track A2) — surface entry for one parametric pad
handle. Carries the feature key (e.g. `"corner_r"`, `"diameter"`),
the resolved sketch parameter name, the current expression string
(read out of `sketch.parameters`), and a UI label so the
Properties panel can render a localised row label without each
editor having to repeat the mapping.
[derive(Debug, Clone)]

## Methods

- `key`
- `label`
- `parameter_name`
- `current_expr`

## Source
Lines 220–233 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
