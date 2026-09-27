---
okf_version: "0.2"
type: Function
title: view
description: "Render the Datasheet tab. Reads / writes `state.row.datasheet`."
resource: crates/oxide-app/src/library/editor/datasheet_picker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/datasheet_picker/view
language: rust
---

# view

Render the Datasheet tab. Reads / writes `state.row.datasheet`.

## Signature

```rust
pub fn view(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the Datasheet tab. Reads / writes `state.row.datasheet`.

## Source
Lines 53–105 in `crates/oxide-app/src/library/editor/datasheet_picker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheet_picker](/crates/oxide-app/src/library/editor/datasheet_picker.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [view_url_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_url_input.md) |
| calls | [view_pinned_input](/crates/oxide-app/src/library/editor/datasheet_picker/view_pinned_input.md) |
