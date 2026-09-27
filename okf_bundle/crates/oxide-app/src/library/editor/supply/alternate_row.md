---
okf_version: "0.2"
type: Function
title: alternate_row
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/alternate_row
language: rust
---

# alternate_row

## Signature

```rust
fn alternate_row(
    idx: usize,
    alt: &'a ManufacturerPart,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 253–353 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [StatusPick](/crates/oxide-app/src/library/editor/supply/StatusPick.md) |
| calls | [remove_button](/crates/oxide-app/src/library/editor/supply/remove_button.md) |
| called_by | [alternates_section](/crates/oxide-app/src/library/editor/supply/alternates_section.md) |
