---
okf_version: "0.2"
type: Function
title: sync_lasso_polygon_to_canvas
description: "Mirror `ui_state.lasso_polygon` into the canvas widget's copy and"
resource: crates/oxide-app/src/app/actions.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/actions/sync_lasso_polygon_to_canvas
language: rust
---

# sync_lasso_polygon_to_canvas

Mirror `ui_state.lasso_polygon` into the canvas widget's copy and

## Signature

```rust
impl Oxide { pub(crate) fn sync_lasso_polygon_to_canvas(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Mirror `ui_state.lasso_polygon` into the canvas widget's copy and
invalidate the overlay cache. Iced's `canvas::Program` only sees
the widget's own state, so the canvas needs its own snapshot; any
mutation to the in-flight lasso must route through this helper so
the two copies never diverge.

## Source
Lines 18–24 in `crates/oxide-app/src/app/actions.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actions](/crates/oxide-app/src/app/actions.md) |
