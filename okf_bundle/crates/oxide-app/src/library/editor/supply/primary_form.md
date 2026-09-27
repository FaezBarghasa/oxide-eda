---
okf_version: "0.2"
type: Function
title: primary_form
description: ─────────────────────────── Primary MPN form ──────────────────────────
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/primary_form
language: rust
---

# primary_form

─────────────────────────── Primary MPN form ──────────────────────────

## Signature

```rust
fn primary_form(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

─────────────────────────── Primary MPN form ──────────────────────────

## Source
Lines 153–220 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [labelled_input](/crates/oxide-app/src/library/editor/supply/labelled_input.md) |
| calls | [StatusPick](/crates/oxide-app/src/library/editor/supply/StatusPick.md) |
