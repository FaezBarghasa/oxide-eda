---
okf_version: "0.2"
type: Function
title: view_pinned_input
resource: crates/oxide-app/src/library/editor/datasheet_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/datasheet_picker/view_pinned_input
language: rust
---

# view_pinned_input

## Signature

```rust
fn view_pinned_input(
    datasheet: Option<&'a DatasheetRef>,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 134–200 in `crates/oxide-app/src/library/editor/datasheet_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet_picker](/crates/oxide-app/src/library/editor/datasheet_picker.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/datasheet_picker/view.md) |
