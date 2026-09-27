---
okf_version: "0.2"
type: Function
title: arc_endpoint_handle_drag_survives_save_reload
description: Dragging an arc endpoint handle into the lower half-plane yields a
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/arc_endpoint_handle_drag_survives_save_reload
language: rust
---

# arc_endpoint_handle_drag_survives_save_reload

Dragging an arc endpoint handle into the lower half-plane yields a

## Signature

```rust
fn arc_endpoint_handle_drag_survives_save_reload()
```

## Decorators

- `test`

## Docstring

Dragging an arc endpoint handle into the lower half-plane yields a
negative `atan2` angle. It must be reduced into `[0, 360)` before it
reaches disk: a raw negative endpoint trips `migrate_legacy_arc`
(which fires on `end_deg < 0.0`) into swapping the pair to its
complement on reload, silently turning the 285° arc the user dragged
into a 75° arc — the same round-trip data loss the Properties-panel
edit path guards against. Regression for the ArcEnd handle writer.
[test]

## Source
Lines 905–943 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| calls | [arc_symbol](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_symbol.md) |
| calls | [move_graphic_handle](/crates/oxide-app/src/library/editor/symbol/state/hit_test/move_graphic_handle.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
