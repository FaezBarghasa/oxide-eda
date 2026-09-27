---
okf_version: "0.2"
type: Function
title: is_tab_switchable
description: "`true` for fields that belong to a multi-field Tab cycle (Line"
resource: crates/oxide-app/src/library/editor/footprint/state/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/placement/is_tab_switchable_1
language: rust
---

# is_tab_switchable

`true` for fields that belong to a multi-field Tab cycle (Line

## Signature

```rust
pub fn is_tab_switchable(self) -> bool
```

## Visibility

- `pub`

## Docstring

`true` for fields that belong to a multi-field Tab cycle (Line
len/angle, Rectangle w/h, Rounded-Rect w/h/radius). Tab cycles
these while a buffer is active; for single-field kinds Tab keeps
its placement-pause role.

## Source
Lines 92–101 in `crates/oxide-app/src/library/editor/footprint/state/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/state/placement.md) |
