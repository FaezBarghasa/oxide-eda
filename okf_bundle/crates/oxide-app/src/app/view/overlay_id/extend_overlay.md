---
okf_version: "0.2"
type: Function
title: extend_overlay
description: "Append `id`'s layers to the stack under construction."
resource: crates/oxide-app/src/app/view/overlay_id.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlay_id/extend_overlay
language: rust
---

# extend_overlay

Append `id`'s layers to the stack under construction.

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn extend_overlay(
        &'a self,
        layers: &mut Vec<Element<'a, Message>>,
        id: OverlayId,
    ) }
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::app::view)`

## Docstring

Append `id`'s layers to the stack under construction.

Each builder keeps its own open/closed guard and returns nothing
when it has nothing to paint; several push two layers (a
click-outside dismiss backdrop, then the card). Both shapes
`extend` the same way, so the arms stay one line each and the
exhaustive match is what guarantees no overlay is silently
dropped from the stack.

## Source
Lines 289–354 in `crates/oxide-app/src/app/view/overlay_id.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay_id](/crates/oxide-app/src/app/view/overlay_id.md) |
