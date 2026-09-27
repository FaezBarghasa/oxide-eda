---
okf_version: "0.2"
type: Function
title: close_x
description: "Same SVG glyph + hover footprint the shared `close_x_button` uses"
resource: crates/oxide-app/src/library/new_component.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/new_component/close_x
language: rust
---

# close_x

Same SVG glyph + hover footprint the shared `close_x_button` uses

## Signature

```rust
fn close_x(message: LibraryMessage, theme_id: ThemeId) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Same SVG glyph + hover footprint the shared `close_x_button` uses
(`view::dialogs::close_x_button`), but generic over message type so
it composes into a `LibraryMessage` element. Matches the OS chrome
close: white glyph, full header-height hit-box, Windows-native red
hover. Kept local because the shared helper is `Message`-typed
only.

## Source
Lines 522–565 in `crates/oxide-app/src/library/new_component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [new_component](/crates/oxide-app/src/library/new_component.md) |
