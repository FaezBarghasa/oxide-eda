---
okf_version: "0.2"
type: Function
title: mouse_interaction
resource: crates/oxide-app/src/app/view/translate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/translate/mouse_interaction_1
language: rust
---

# mouse_interaction

## Signature

```rust
fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction
```

## Source
Lines 120–141 in `crates/oxide-app/src/app/view/translate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [translate](/crates/oxide-app/src/app/view/translate.md) |
