---
okf_version: "0.2"
type: Function
title: floating_panel_shadow
description: Floating panel outer wrapper (shadow only)
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles/floating_panel_shadow
language: rust
---

# floating_panel_shadow

Floating panel outer wrapper (shadow only)

## Signature

```rust
pub fn floating_panel_shadow(
    _tokens: &ThemeTokens,
) -> impl Fn(&Theme) -> container::Style + 'static
```

## Visibility

- `pub`

## Docstring

Floating panel outer wrapper (shadow only)

## Source
Lines 293–304 in `crates/oxide-app/src/styles.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [styles](/crates/oxide-app/src/styles.md) |
| called_by | [view_floating_panel](/crates/oxide-app/src/dock/view/view_floating_panel.md) |
