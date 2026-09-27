---
okf_version: "0.2"
type: Function
title: labelled_input
description: ─────────────────────────── Shared widgets ────────────────────────────
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/labelled_input
language: rust
---

# labelled_input

─────────────────────────── Shared widgets ────────────────────────────

## Signature

```rust
fn labelled_input(
    label: &'static str,
    value: String,
    placeholder: &'static str,
    msg: fn(String) -> EditorMsg,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

─────────────────────────── Shared widgets ────────────────────────────

## Source
Lines 504–531 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| called_by | [primary_form](/crates/oxide-app/src/library/editor/supply/primary_form.md) |
