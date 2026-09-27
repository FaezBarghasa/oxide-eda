---
okf_version: "0.2"
type: Function
title: placement_paused_suppresses_rounded_rect_commit_click
description: "v0.14-footprint #23 — while `placement_paused` is set, a sketch"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_paused_suppresses_rounded_rect_commit_click
language: rust
---

# placement_paused_suppresses_rounded_rect_commit_click

v0.14-footprint #23 — while `placement_paused` is set, a sketch

## Signature

```rust
fn placement_paused_suppresses_rounded_rect_commit_click()
```

## Decorators

- `test`

## Docstring

v0.14-footprint #23 — while `placement_paused` is set, a sketch
commit click must be dropped before it can advance `tool_pending`
or mint geometry. Reproduces the "TAB shows paused but the Rounded
Rectangle still commits" bug: the canvas-layer gate leaked the
commit click, so the authoritative gate now lives at the top of
the sketch-click dispatcher arm.
[test]

## Source
Lines 782–880 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
