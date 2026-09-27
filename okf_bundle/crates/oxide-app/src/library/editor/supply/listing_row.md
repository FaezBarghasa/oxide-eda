---
okf_version: "0.2"
type: Function
title: listing_row
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/listing_row
language: rust
---

# listing_row

## Signature

```rust
fn listing_row(
    idx: usize,
    distributor_str: &str,
    sku: &str,
    url: &str,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 427–500 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [DistributorPick](/crates/oxide-app/src/library/editor/supply/DistributorPick.md) |
| calls | [distributor_from_label](/crates/oxide-app/src/library/editor/supply/distributor_from_label.md) |
| calls | [remove_button](/crates/oxide-app/src/library/editor/supply/remove_button.md) |
| called_by | [listings_section](/crates/oxide-app/src/library/editor/supply/listings_section.md) |
