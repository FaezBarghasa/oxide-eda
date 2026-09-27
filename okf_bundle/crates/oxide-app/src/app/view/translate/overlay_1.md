---
okf_version: "0.2"
type: Function
title: overlay
resource: crates/oxide-app/src/app/view/translate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/translate/overlay_1
language: rust
---

# overlay

## Signature

```rust
fn overlay(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout<'a>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>>
```

## Type Parameters

- `'a`

## Source
Lines 169–182 in `crates/oxide-app/src/app/view/translate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [translate](/crates/oxide-app/src/app/view/translate.md) |
