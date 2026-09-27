---
okf_version: "0.2"
type: Function
title: a_pending_fit_reaches_the_camera_in_one_hop
description: "#632 — a fit request used to travel `CanvasSlot::pending_fit` →"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/a_pending_fit_reaches_the_camera_in_one_hop
language: rust
---

# a_pending_fit_reaches_the_camera_in_one_hop

#632 — a fit request used to travel `CanvasSlot::pending_fit` →

## Signature

```rust
fn a_pending_fit_reaches_the_camera_in_one_hop()
```

## Decorators

- `test`

## Docstring

#632 — a fit request used to travel `CanvasSlot::pending_fit` →
`CanvasState::pending_fit` → camera, two `take()`s deep, because the
camera lived in the state the first hop was reaching. With one camera
home the middle hop has no reader, so the request must be consumed and
applied in a single `update_pending_fit` call.
[test]

## Source
Lines 808–839 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
| calls | [test_prefs](/crates/oxide-app/src/canvas/mod/test_prefs.md) |
