---
okf_version: "0.2"
type: Function
title: danger_button_style
description: Destructive button (Discard / Delete / Remove / Stop) — theme danger
resource: crates/oxide-app/src/preferences/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/widgets/danger_button_style
language: rust
---

# danger_button_style

Destructive button (Discard / Delete / Remove / Stop) — theme danger

## Signature

```rust
pub(super) fn danger_button_style(theme: &Theme, status: button::Status) -> button::Style
```

## Visibility

- `pub(super)`

## Docstring

Destructive button (Discard / Delete / Remove / Stop) — theme danger
fill with its guaranteed-contrast text colour.

## Source
Lines 68–83 in `crates/oxide-app/src/preferences/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/preferences/widgets.md) |
