---
okf_version: "0.2"
type: Function
title: view
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/view
language: rust
---

# view

## Signature

```rust
pub fn view(
    state: &'a ComponentPreviewState,
    library_state: &'a LibraryState,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Source
Lines 49–148 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [resolve_template](/crates/oxide-app/src/library/editor/params/resolve_template.md) |
| calls | [template_row](/crates/oxide-app/src/library/editor/params/template_row.md) |
| calls | [custom_row](/crates/oxide-app/src/library/editor/params/custom_row.md) |
| calls | [add_custom_row](/crates/oxide-app/src/library/editor/params/add_custom_row.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
