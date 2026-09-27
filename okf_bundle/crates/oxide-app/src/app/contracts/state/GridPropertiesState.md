---
okf_version: "0.2"
type: Class
title: GridPropertiesState
description: v0.18.11 — Cartesian Grid Editor modal state. Carries the
resource: crates/oxide-app/src/app/contracts/state.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/contracts/state/GridPropertiesState
language: rust
---

# GridPropertiesState

v0.18.11 — Cartesian Grid Editor modal state. Carries the

## Signature

```rust
pub struct GridPropertiesState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.18.11 — Cartesian Grid Editor modal state. Carries the
in-flight Step X / Step Y string buffers + the X/Y link toggle.
Writes happen on `GridPropertiesApply`; close discards.

v0.18.19 added Fine / Coarse display + Multiplier draft fields.
[derive(Debug, Clone)]

## Methods

- `step_x_mm`
- `step_y_mm`
- `link_xy`
- `fine_display`
- `coarse_display`
- `multiplier`

## Source
Lines 76–83 in `crates/oxide-app/src/app/contracts/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/contracts/state.md) |
