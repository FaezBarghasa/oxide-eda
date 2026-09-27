---
okf_version: "0.2"
type: Function
title: view_messages
description: ─── Messages Panel ───────────────────────────────────────────
resource: crates/oxide-app/src/panels/status.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/status/view_messages
language: rust
---

# view_messages

─── Messages Panel ───────────────────────────────────────────

## Signature

```rust
pub fn view_messages(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

─── Messages Panel ───────────────────────────────────────────

## Source
Lines 220–344 in `crates/oxide-app/src/panels/status.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status](/crates/oxide-app/src/panels/status.md) |
| calls | [success_color](/crates/oxide-widgets/src/theme_ext/success_color.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [error_color](/crates/oxide-widgets/src/theme_ext/error_color.md) |
| calls | [warning_color](/crates/oxide-widgets/src/theme_ext/warning_color.md) |
| calls | [accent](/crates/oxide-widgets/src/theme_ext/accent.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
