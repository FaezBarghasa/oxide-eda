---
okf_version: "0.2"
type: Function
title: preview_panel_with_pick
resource: crates/oxide-app/src/library/browser/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/preview/preview_panel_with_pick
language: rust
---

# preview_panel_with_pick

## Signature

```rust
fn preview_panel_with_pick(
    label: &'a str,
    summary: String,
    pick_label: &'a str,
    pick_msg: LibraryMessage,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    dead_code,
    reason = "F15 removed the preview pane; the builders stay until the Properties panel absorbs them"
)`

## Source
Lines 214–266 in `crates/oxide-app/src/library/browser/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/browser/preview.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
