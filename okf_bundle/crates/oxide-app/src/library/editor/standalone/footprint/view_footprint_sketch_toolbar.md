---
okf_version: "0.2"
type: Function
title: view_footprint_sketch_toolbar
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_sketch_toolbar
language: rust
---

# view_footprint_sketch_toolbar

## Signature

```rust
fn view_footprint_sketch_toolbar(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    dead_code,
    reason = "standalone footprint chrome superseded by the Active Bar; kept for one migration cycle"
)`

## Source
Lines 338–413 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [tab_bar_strip](/crates/oxide-app/src/styles/tab_bar_strip.md) |
