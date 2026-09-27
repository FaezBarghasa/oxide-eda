---
okf_version: "0.2"
type: Function
title: view_footprint_footer
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_footer
language: rust
---

# view_footprint_footer

## Signature

```rust
fn view_footprint_footer(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 704–741 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| called_by | [view_footprint](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint.md) |
