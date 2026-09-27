---
okf_version: "0.2"
type: Function
title: operate
resource: crates/oxide-app/src/app/view/translate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/translate/operate
language: rust
---

# operate

## Signature

```rust
impl Translate<'_, Message, Theme, Renderer> { fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) }
```

## Type Parameters

- `Message`
- `Theme`
- `Renderer`

## Source
Lines 81–93 in `crates/oxide-app/src/app/view/translate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [translate](/crates/oxide-app/src/app/view/translate.md) |
