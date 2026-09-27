---
okf_version: "0.2"
type: Function
title: listings_section
description: ──────────────────────── Distributor listings ─────────────────────────
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/listings_section
language: rust
---

# listings_section

──────────────────────── Distributor listings ─────────────────────────

## Signature

```rust
fn listings_section(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

──────────────────────── Distributor listings ─────────────────────────

## Source
Lines 357–425 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [listing_row](/crates/oxide-app/src/library/editor/supply/listing_row.md) |
