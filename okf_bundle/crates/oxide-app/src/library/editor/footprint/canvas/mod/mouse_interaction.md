---
okf_version: "0.2"
type: Function
title: mouse_interaction
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod/mouse_interaction
language: rust
---

# mouse_interaction

## Signature

```rust
impl FootprintCanvas<'a> { fn mouse_interaction(
        &self,
        cstate: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction }
```

## Type Parameters

- `'a`

## Source
Lines 314–469 in `crates/oxide-app/src/library/editor/footprint/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
