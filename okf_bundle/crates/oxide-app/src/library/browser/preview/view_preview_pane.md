---
okf_version: "0.2"
type: Function
title: view_preview_pane
resource: crates/oxide-app/src/library/browser/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/preview/view_preview_pane
language: rust
---

# view_preview_pane

## Signature

```rust
fn view_preview_pane(
    library_path: &'a std::path::Path,
    table: &str,
    library_state: &'a LibraryState,
    visible: &[&'a ComponentRow],
    selected: Option<RowId>,
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
Lines 17–157 in `crates/oxide-app/src/library/browser/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/browser/preview.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| calls | [symbol_summary](/crates/oxide-app/src/library/browser/preview/symbol_summary.md) |
| calls | [footprint_summary](/crates/oxide-app/src/library/browser/preview/footprint_summary.md) |
| calls | [preview_panel_with_pick](/crates/oxide-app/src/library/browser/preview/preview_panel_with_pick.md) |
