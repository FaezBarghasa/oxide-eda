---
okf_version: "0.2"
type: Function
title: color_row
description: Render one color row — preview swatch + 4 numeric inputs (R/G/B/A
resource: crates/oxide-app/src/library/editor/footprint/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d/color_row
language: rust
---

# color_row

Render one color row — preview swatch + 4 numeric inputs (R/G/B/A

## Signature

```rust
fn color_row(
    label: &'static str,
    rgba: [f32; 4],
    muted: iced::Color,
    text_c: iced::Color,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
    is_top: bool,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Render one color row — preview swatch + 4 numeric inputs (R/G/B/A
0..1) + cycle button. We don't pull the full ColorPicker here
(would explode the panel height); the cycle preset gives the user
quick access to a dark/light/custom palette.

## Source
Lines 154–237 in `crates/oxide-app/src/library/editor/footprint/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-app/src/library/editor/footprint/body3d.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/body3d/view.md) |
