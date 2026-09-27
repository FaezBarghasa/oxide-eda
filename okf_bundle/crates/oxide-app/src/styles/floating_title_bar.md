---
okf_version: "0.2"
type: Function
title: floating_title_bar
description: Floating panel title bar
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/floating_title_bar
language: rust
---

# floating_title_bar

Floating panel title bar

## Signature

```rust
pub fn floating_title_bar(tokens: &ThemeTokens) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Floating panel title bar

## Source
Lines 263–275 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_floating_panel](/crates/oxide-app/src/dock/view/view_floating_panel.md) |
