---
okf_version: "0.2"
type: Function
title: text_warning
description: "Warning text (unsaved-changes marker, invalid / conflicting bindings)."
resource: crates/oxide-app/src/preferences/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/widgets/text_warning
language: rust
---

# text_warning

Warning text (unsaved-changes marker, invalid / conflicting bindings).

## Signature

```rust
pub(super) fn text_warning(theme: &Theme) -> text::Style
```

## Visibility

- `pub(super)`

## Docstring

Warning text (unsaved-changes marker, invalid / conflicting bindings).
iced's `ExtendedPalette` carries no warning token, but the base
`Palette` exposes a caution hue that is semantic and reads correctly on
both dark and light backgrounds — so we resolve it from there.

## Source
Lines 41–45 in `crates/oxide-app/src/preferences/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/preferences/widgets.md) |
