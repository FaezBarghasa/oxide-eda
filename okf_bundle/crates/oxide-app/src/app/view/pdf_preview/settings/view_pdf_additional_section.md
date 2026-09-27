---
okf_version: "0.2"
type: Function
title: view_pdf_additional_section
description: Settings → Additional PDF Settings.
resource: crates/oxide-app/src/app/view/pdf_preview/settings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_additional_section
language: rust
---

# view_pdf_additional_section

Settings → Additional PDF Settings.

## Signature

```rust
impl Oxide { fn view_pdf_additional_section(
        &self,
        preview: &crate::app::state::PreviewState,
    ) -> Element<'_, Message> }
```

## Docstring

Settings → Additional PDF Settings.

## Source
Lines 225–418 in `crates/oxide-app/src/app/view/pdf_preview/settings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [settings](/crates/oxide-app/src/app/view/pdf_preview/settings.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
