---
okf_version: "0.2"
type: Function
title: custom_row
description: "Layout a custom parameter row — same value editor as `template_row`"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/custom_row
language: rust
---

# custom_row

Layout a custom parameter row — same value editor as `template_row`

## Signature

```rust
fn custom_row(
    state: &'a ComponentPreviewState,
    name: &'a str,
    val: &'a ParamValue,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Layout a custom parameter row — same value editor as `template_row`
but with a per-row `[×]` remove button instead of the missing badge.

## Source
Lines 202–279 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [slot_input](/crates/oxide-app/src/library/editor/params/slot_input.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
