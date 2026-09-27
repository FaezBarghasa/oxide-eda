---
okf_version: "0.2"
type: Module
title: icons
description: Central icon registry with runtime theme-aware tinting.
resource: crates/oxide-app/src/icons.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/icons
language: rust
---

# icons

Central icon registry with runtime theme-aware tinting.

## Docstring

Central icon registry with runtime theme-aware tinting.

Design
------
There is **one canonical SVG tree** at `assets/icons/…`. Every accent
path in those SVGs uses the Oxide brand amber `#f59e0b` as a
sentinel colour. At fetch time the sentinel is string-replaced with
the current theme's accent hex and the resulting bytes are handed to
`iced::widget::svg::Handle::from_memory`, which de-duplicates by
content hash — so repeat renders of the same (icon, theme) pair reuse
the GPU texture cache and the replace cost is paid once.

For themes that only change the **accent colour**, no icon copy is
needed: the `canonical_icon!` macro expands to a tint-only lookup.

For a theme that needs a genuinely **different shape** for a specific
icon (logo-style, brand-dependent glyph, …), hand-write the function
with an explicit match arm that pulls from a per-theme override
directory. Example:

```ignore
pub fn icon_logo(theme: ThemeId) -> svg::Handle {
match theme {
ThemeId::Alplab => {
// Shape override — cyan-baked SVG with different strokes.
svg::Handle::from_memory(
include_bytes!("../assets/icons/alplab/logo.svg").as_slice(),
)
}
_ => canonical("logo.svg", theme),
}
}
```

The reserved `assets/icons/alplab/` tree exists as a pre-tinted
starting point for authoring such overrides.

## Relationships

| Type | Target |
|------|--------|
| related | [accent_hex](/crates/oxide-app/src/icons/accent_hex.md) |
| related | [tinted_handle](/crates/oxide-app/src/icons/tinted_handle.md) |
