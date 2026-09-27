---
okf_version: "0.2"
type: Function
title: alternates_section
description: ────────────────────────── Alternates section ─────────────────────────
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/alternates_section
language: rust
---

# alternates_section

────────────────────────── Alternates section ─────────────────────────

## Signature

```rust
fn alternates_section(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

────────────────────────── Alternates section ─────────────────────────

## Source
Lines 224–251 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [alternate_row](/crates/oxide-app/src/library/editor/supply/alternate_row.md) |
| calls | [add_button](/crates/oxide-app/src/library/editor/supply/add_button.md) |
