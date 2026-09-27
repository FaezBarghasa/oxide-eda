---
okf_version: "0.2"
type: Function
title: secondary_button_style
description: Neutral secondary button (Import / Export / Edit / Cancel / Clear) —
resource: crates/oxide-app/src/preferences/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/widgets/secondary_button_style
language: rust
---

# secondary_button_style

Neutral secondary button (Import / Export / Edit / Cancel / Clear) —

## Signature

```rust
pub(super) fn secondary_button_style(theme: &Theme, status: button::Status) -> button::Style
```

## Visibility

- `pub(super)`

## Docstring

Neutral secondary button (Import / Export / Edit / Cancel / Clear) —
theme-derived so it reads on both light and dark palettes.

## Source
Lines 136–152 in `crates/oxide-app/src/preferences/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/preferences/widgets.md) |
