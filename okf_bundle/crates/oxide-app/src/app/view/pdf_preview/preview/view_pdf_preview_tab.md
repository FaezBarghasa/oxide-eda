---
okf_version: "0.2"
type: Function
title: view_pdf_preview_tab
description: "Preview tab — top toolbar (Sheet/Colour/Pages/Output), thumb"
resource: crates/oxide-app/src/app/view/pdf_preview/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/view/pdf_preview/preview/view_pdf_preview_tab
language: rust
---

# view_pdf_preview_tab

Preview tab — top toolbar (Sheet/Colour/Pages/Output), thumb

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_pdf_preview_tab(
        &self,
        preview: &crate::app::state::PreviewState,
    ) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Preview tab — top toolbar (Sheet/Colour/Pages/Output), thumb
rail on the left, pan/zoom viewport on the right.

## Source
Lines 10–366 in `crates/oxide-app/src/app/view/pdf_preview/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/app/view/pdf_preview/preview.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
