---
okf_version: "0.2"
type: Function
title: pad_stack_tab_strip
description: v0.20 — Pad Stack tab strip (Simple / Top-Middle-Bottom / Full
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/pad_stack_tab_strip
language: rust
---

# pad_stack_tab_strip

v0.20 — Pad Stack tab strip (Simple / Top-Middle-Bottom / Full

## Signature

```rust
pub(super) fn pad_stack_tab_strip(
    values: &PadFormValues,
    primary: Color,
    muted: Color,
    border_c: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — Pad Stack tab strip (Simple / Top-Middle-Bottom / Full
Stack). UI-only structure today; per-layer overrides require a
v0.21 schema follow-up so the body stays the same across tabs.

## Source
Lines 442–499 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
