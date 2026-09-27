---
okf_version: "0.2"
type: Function
title: view_header
resource: crates/oxide-app/src/library/editor/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/mod/view_header
language: rust
---

# view_header

## Signature

```rust
fn view_header(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 61–89 in `crates/oxide-app/src/library/editor/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/library/editor/mod.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/mod/view.md) |
