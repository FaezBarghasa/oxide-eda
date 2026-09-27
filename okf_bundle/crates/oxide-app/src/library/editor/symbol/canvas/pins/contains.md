---
okf_version: "0.2"
type: Function
title: contains
description: "True when `(x, y)` lies inside (or on the edge of) the box."
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins/contains
language: rust
---

# contains

True when `(x, y)` lies inside (or on the edge of) the box.

## Signature

```rust
impl Aabb { pub(super) fn contains(&self, x: f64, y: f64) -> bool }
```

## Visibility

- `pub(super)`

## Docstring

True when `(x, y)` lies inside (or on the edge of) the box.

## Source
Lines 91–93 in `crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pins](/crates/oxide-app/src/library/editor/symbol/canvas/pins.md) |
