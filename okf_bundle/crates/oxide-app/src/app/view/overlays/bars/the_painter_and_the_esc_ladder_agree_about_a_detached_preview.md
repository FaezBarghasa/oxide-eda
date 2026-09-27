---
okf_version: "0.2"
type: Function
title: the_painter_and_the_esc_ladder_agree_about_a_detached_preview
description: The Esc ladder builds its own copy of this predicate from a
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/the_painter_and_the_esc_ladder_agree_about_a_detached_preview
language: rust
---

# the_painter_and_the_esc_ladder_agree_about_a_detached_preview

The Esc ladder builds its own copy of this predicate from a

## Signature

```rust
fn the_painter_and_the_esc_ladder_agree_about_a_detached_preview()
```

## Decorators

- `test`

## Docstring

The Esc ladder builds its own copy of this predicate from a
snapshot (`OpenOverlays::has_blocking_modal`,
`app/bootstrap/subscription.rs`). If the two disagree, Esc
resolves against a stack that is not on screen — which is exactly
what #547 was. Pin the agreement on the term that diverged.
[test]

## Source
Lines 814–836 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
| calls | [blank_preview](/crates/oxide-app/src/app/view/overlays/bars/blank_preview.md) |
