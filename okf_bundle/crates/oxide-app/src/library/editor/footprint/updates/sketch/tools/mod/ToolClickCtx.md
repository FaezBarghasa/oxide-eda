---
okf_version: "0.2"
type: Class
title: ToolClickCtx
description: "The per-click state the tool sub-modules read: the sketch plane, the"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/ToolClickCtx
language: rust
---

# ToolClickCtx

The per-click state the tool sub-modules read: the sketch plane, the

## Signature

```rust
pub(super) struct ToolClickCtx
```

## Visibility

- `pub(super)`

## Docstring

The per-click state the tool sub-modules read: the sketch plane, the
resolved (snapped or freshly-minted) click Point, the raw click position,
and the sticky construction / centerline flags applied to new entities.

## Methods

- `plane_id`
- `resolved_id`
- `x_mm`
- `y_mm`
- `construction_mode`
- `centerline_mode`

## Source
Lines 25–32 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
