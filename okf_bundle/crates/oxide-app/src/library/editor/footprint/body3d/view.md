---
okf_version: "0.2"
type: Function
title: view
description: "Render the Body 3D editor pane. `body` is borrowed from"
resource: crates/oxide-app/src/library/editor/footprint/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d/view
language: rust
---

# view

Render the Body 3D editor pane. `body` is borrowed from

## Signature

```rust
pub fn view(
    body: &Body3D,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Body 3D editor pane. `body` is borrowed from
`Footprint::body_3d`; messages mutate it through the dispatcher.

## Source
Lines 37–134 in `crates/oxide-app/src/library/editor/footprint/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-app/src/library/editor/footprint/body3d.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [ShapePick](/crates/oxide-app/src/library/editor/footprint/body3d/ShapePick.md) |
| calls | [color_row](/crates/oxide-app/src/library/editor/footprint/body3d/color_row.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
