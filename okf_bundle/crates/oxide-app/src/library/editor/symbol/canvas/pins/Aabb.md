---
okf_version: "0.2"
type: Class
title: Aabb
description: A world-mm axis-aligned bounding box used for label hit-testing.
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins/Aabb
language: rust
---

# Aabb

A world-mm axis-aligned bounding box used for label hit-testing.

## Signature

```rust
pub(super) struct Aabb
```

## Visibility

- `pub(super)`

## Docstring

A world-mm axis-aligned bounding box used for label hit-testing.
A degenerate box (an empty label) sets `min > max` so it can never
contain any point.

## Methods

- `min`
- `max`

## Source
Lines 84–87 in `crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pins](/crates/oxide-app/src/library/editor/symbol/canvas/pins.md) |
