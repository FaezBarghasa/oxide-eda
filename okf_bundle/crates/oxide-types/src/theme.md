---
okf_version: "0.2"
type: Module
title: theme
description: Built-in themes for the Oxide EDA application.
resource: crates/oxide-types/src/theme.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/theme
language: rust
---

# theme

Built-in themes for the Oxide EDA application.

## Docstring

Built-in themes for the Oxide EDA application.

# The one central theme source

This module is the **single source of truth** for every colour in Oxide.
[`ThemeId`] selects a theme; [`ThemeTokens`] holds the UI-chrome palette
(backgrounds, text, accent, panels, status colours) and [`CanvasColors`]
holds the drawing palette (wire, junction, body, pad, silk, …). Nothing
else defines colours — every surface *consumes* these tokens:

- `oxide-widgets` (`theme_ext.rs`) bridges tokens into per-widget iced
[`Style`](https://docs.rs/iced/0.14/iced/widget/container/struct.Style.html)
values — the iced 0.14 Catalog route. iced's own `Theme`/`Palette` is
therefore just *one consumer* of these tokens, not a second source.
- `oxide-renderer` / the `oxide-gfx` wgpu path read the same tokens for
the GPU/canvas draw colours.
- `oxide-output` reads them for SVG / PDF export.
- `oxide-app` and `chrome-catalog` read them for the shell UI.

This crate has **zero `iced` / `wgpu` dependency** on purpose (ADR: separate
types from rendering): because the tokens live in the domain layer, the
non-iced surfaces (GPU shader, export) can share them, so the wgpu PCB
renderer and the SVG/PDF export stay in lock-step with the iced UI. A
re-skin is one edit to [`ThemeId`]'s token tables; every surface follows.

## Relationships

| Type | Target |
|------|--------|
| related | [Color](/crates/oxide-types/src/theme/Color.md) |
| related | [new](/crates/oxide-types/src/theme/new.md) |
| related | [from_hex](/crates/oxide-types/src/theme/from_hex.md) |
| related | [new](/crates/oxide-types/src/theme/new.md) |
| related | [from_hex](/crates/oxide-types/src/theme/from_hex.md) |
| related | [ThemeId](/crates/oxide-types/src/theme/ThemeId.md) |
| related | [label](/crates/oxide-types/src/theme/label.md) |
| related | [label](/crates/oxide-types/src/theme/label.md) |
| related | [CustomThemeFile](/crates/oxide-types/src/theme/CustomThemeFile.md) |
| related | [ThemeTokens](/crates/oxide-types/src/theme/ThemeTokens.md) |
| related | [CanvasColors](/crates/oxide-types/src/theme/CanvasColors.md) |
| related | [c](/crates/oxide-types/src/theme/c.md) |
| related | [theme_tokens](/crates/oxide-types/src/theme/theme_tokens.md) |
| related | [canvas_colors](/crates/oxide-types/src/theme/canvas_colors.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
