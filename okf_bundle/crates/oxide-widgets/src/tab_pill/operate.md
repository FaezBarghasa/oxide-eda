---
okf_version: "0.2"
type: Function
title: operate
resource: crates/oxide-widgets/src/tab_pill.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/tab_pill/operate
language: rust
---

# operate

## Signature

```rust
impl TabPill<'_, Message, Theme, Renderer> { fn operate(
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
Lines 130–142 in `crates/oxide-widgets/src/tab_pill.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_pill](/crates/oxide-widgets/src/tab_pill.md) |
