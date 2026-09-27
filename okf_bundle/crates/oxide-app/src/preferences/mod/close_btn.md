---
okf_version: "0.2"
type: Function
title: close_btn
description: Canonical close-X — same SVG glyph and red hover footprint as the
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/close_btn
language: rust
---

# close_btn

Canonical close-X — same SVG glyph and red hover footprint as the

## Signature

```rust
fn close_btn(theme_id: ThemeId) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Docstring

Canonical close-X — same SVG glyph and red hover footprint as the
shared `view::dialogs::close_x_button`, generic over the modal's
message type so this version can compose into a `PrefMsg`
element. Matches the main-window chrome close (white glyph,
46×28 hit-box, top-right rounded hover, Windows-native red).

## Source
Lines 627–670 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
