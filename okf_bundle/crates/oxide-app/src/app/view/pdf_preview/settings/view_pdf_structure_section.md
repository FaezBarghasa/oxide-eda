---
okf_version: "0.2"
type: Function
title: view_pdf_structure_section
description: Settings → Structure Settings.
resource: crates/oxide-app/src/app/view/pdf_preview/settings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/pdf_preview/settings/view_pdf_structure_section
language: rust
---

# view_pdf_structure_section

Settings → Structure Settings.

## Signature

```rust
impl Oxide { fn view_pdf_structure_section(
        &self,
        preview: &crate::app::state::PreviewState,
    ) -> Element<'_, Message> }
```

## Docstring

Settings → Structure Settings.

## Source
Lines 151–222 in `crates/oxide-app/src/app/view/pdf_preview/settings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [settings](/crates/oxide-app/src/app/view/pdf_preview/settings.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
