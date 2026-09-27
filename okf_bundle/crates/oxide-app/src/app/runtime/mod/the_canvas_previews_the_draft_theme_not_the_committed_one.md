---
okf_version: "0.2"
type: Function
title: the_canvas_previews_the_draft_theme_not_the_committed_one
description: Picking a theme in Preferences previews it on the canvas before
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/the_canvas_previews_the_draft_theme_not_the_committed_one
language: rust
---

# the_canvas_previews_the_draft_theme_not_the_committed_one

Picking a theme in Preferences previews it on the canvas before

## Signature

```rust
fn the_canvas_previews_the_draft_theme_not_the_committed_one()
```

## Decorators

- `test`

## Docstring

Picking a theme in Preferences previews it on the canvas before
Save. That preview used to be a hand-written push of the computed
colours onto the canvas from the `DraftTheme` arm; it is now the
draft field itself feeding `canvas_view_prefs`. If this reverted to
reading the committed `theme_id`, the picker would look dead.
[test]

## Source
Lines 313–336 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
