---
okf_version: "0.2"
type: Function
title: add_custom_row
description: "Inline \"+ Add custom parameter\" row — text input + four kind buttons"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/add_custom_row
language: rust
---

# add_custom_row

Inline "+ Add custom parameter" row — text input + four kind buttons

## Signature

```rust
fn add_custom_row(
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Inline "+ Add custom parameter" row — text input + four kind buttons
(Text / Number / Bool / Measurement). The kind buttons live as
separate `+` buttons because adding a custom row is rare and a
pick-list would be heavier than the four-pill UI.

The "Add" buttons read the buffered name from the editor's
`params_edit_buf` map under the sentinel key "" (empty string, kept
out of the displayed-rows pass because parameters with empty names
are rejected by the dispatcher).

## Source
Lines 441–510 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
