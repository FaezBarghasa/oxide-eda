---
okf_version: "0.2"
type: Function
title: mouse_interaction
resource: crates/oxide-widgets/src/tab_pill.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/tab_pill/mouse_interaction
language: rust
---

# mouse_interaction

## Signature

```rust
impl TabPill<'_, Message, Theme, Renderer> { fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction }
```

## Type Parameters

- `Message`
- `Theme`
- `Renderer`

## Source
Lines 169–190 in `crates/oxide-widgets/src/tab_pill.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_pill](/crates/oxide-widgets/src/tab_pill.md) |
