---
okf_version: "0.2"
type: Function
title: primary_button_style
description: Primary action button (Import / Export / Add / Reset-ERC) — theme accent
resource: crates/oxide-app/src/preferences/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/widgets/primary_button_style
language: rust
---

# primary_button_style

Primary action button (Import / Export / Add / Reset-ERC) — theme accent

## Signature

```rust
pub(super) fn primary_button_style(theme: &Theme, status: button::Status) -> button::Style
```

## Visibility

- `pub(super)`

## Docstring

Primary action button (Import / Export / Add / Reset-ERC) — theme accent
fill with its guaranteed-contrast text colour.

## Source
Lines 49–64 in `crates/oxide-app/src/preferences/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/preferences/widgets.md) |
