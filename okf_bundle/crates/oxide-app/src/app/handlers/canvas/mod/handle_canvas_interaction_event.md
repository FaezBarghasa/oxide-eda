---
okf_version: "0.2"
type: Function
title: handle_canvas_interaction_event
resource: crates/oxide-app/src/app/handlers/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/mod/handle_canvas_interaction_event
language: rust
---

# handle_canvas_interaction_event

## Signature

```rust
impl Oxide { pub(crate) fn handle_canvas_interaction_event(&mut self, event: CanvasEvent) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 43–238 in `crates/oxide-app/src/app/handlers/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/app/handlers/canvas/mod.md) |
| calls | [primary_anchor_world](/crates/oxide-app/src/app/handlers/canvas/mod/primary_anchor_world.md) |
| calls | [passes_filter](/crates/oxide-app/src/app/handlers/selection_workflow/passes_filter.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
