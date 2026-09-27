---
okf_version: "0.2"
type: Function
title: chrome_separator
description: 1 px horizontal divider that sits between two strips of chrome —
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/chrome_separator
language: rust
---

# chrome_separator

1 px horizontal divider that sits between two strips of chrome —

## Signature

```rust
pub fn chrome_separator(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

1 px horizontal divider that sits between two strips of chrome —
used between the menu/Active-Bar row and the document tab strip
so the two zones read as separate UI bands.

## Source
Lines 41–47 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| called_by | [view_main_for](/crates/oxide-app/src/app/view/mod/view_main_for.md) |
