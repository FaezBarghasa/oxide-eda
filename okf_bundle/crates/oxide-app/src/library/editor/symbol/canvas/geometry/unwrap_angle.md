---
okf_version: "0.2"
type: Function
title: unwrap_angle
description: "Unwrap a raw `atan2` angle (in degrees, range `[-180, 180]`) so that"
resource: crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/geometry/unwrap_angle
language: rust
---

# unwrap_angle

Unwrap a raw `atan2` angle (in degrees, range `[-180, 180]`) so that

## Signature

```rust
pub(super) fn unwrap_angle(prev: f64, raw: f64) -> f64
```

## Visibility

- `pub(super)`

## Docstring

Unwrap a raw `atan2` angle (in degrees, range `[-180, 180]`) so that
the result stays within 180° of `prev`. This removes the ±180° branch
cut when tracking a continuously-moving cursor angle.

Example: prev = 170°, raw = -170° → returns 190° (not -170°).

## Source
Lines 30–40 in `crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/symbol/canvas/geometry.md) |
| called_by | [on_cursor_moved](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_cursor_moved.md) |
