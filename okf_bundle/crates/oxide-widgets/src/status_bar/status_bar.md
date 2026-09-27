---
okf_version: "0.2"
type: Function
title: status_bar
description: Render a themed status bar.
resource: crates/oxide-widgets/src/status_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/status_bar/status_bar
language: rust
---

# status_bar

Render a themed status bar.

## Signature

```rust
pub fn status_bar(
    left: &[StatusSection],
    right: &[StatusSection],
    tokens: &ThemeTokens,
) -> Element<'a, StatusBarMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render a themed status bar.

* `left`   — sections aligned to the left.
* `right`  — sections aligned to the right.
* `tokens` — theme tokens for all colors.

## Source
Lines 56–124 in `crates/oxide-widgets/src/status_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status_bar](/crates/oxide-widgets/src/status_bar.md) |
| calls | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [success_color](/crates/oxide-widgets/src/theme_ext/success_color.md) |
| calls | [render_section](/crates/oxide-widgets/src/status_bar/render_section.md) |
| calls | [to_color](/crates/oxide-widgets/src/theme_ext/to_color.md) |
