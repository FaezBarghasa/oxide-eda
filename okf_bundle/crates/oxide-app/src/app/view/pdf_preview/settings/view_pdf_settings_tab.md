---
okf_version: "0.2"
type: Function
title: view_pdf_settings_tab
description: Settings tab — stitches the three section helpers below into a
resource: crates/oxide-app/src/app/view/pdf_preview/settings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_settings_tab
language: rust
---

# view_pdf_settings_tab

Settings tab — stitches the three section helpers below into a

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_pdf_settings_tab(
        &self,
        preview: &crate::app::state::PreviewState,
    ) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Settings tab — stitches the three section helpers below into a
single scrollable column. Each helper owns its own widgets and
reads/writes through `preview.pdf_options.*` directly so the
rasterizer and exporter stay in lockstep with the UI.

## Source
Lines 12–30 in `crates/oxide-app/src/app/view/pdf_preview/settings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [settings](/crates/oxide-app/src/app/view/pdf_preview/settings.md) |
