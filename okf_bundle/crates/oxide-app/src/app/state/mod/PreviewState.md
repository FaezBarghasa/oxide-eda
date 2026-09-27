---
okf_version: "0.2"
type: Class
title: PreviewState
description: Open-print-preview state — rasterised pages + which one is currently
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/PreviewState
language: rust
---

# PreviewState

Open-print-preview state — rasterised pages + which one is currently

## Signature

```rust
pub struct PreviewState
```

## Visibility

- `pub`

## Docstring

Open-print-preview state — rasterised pages + which one is currently
shown full-size. Pages are produced by `oxide_output::PreviewRasterizer`
when the user invokes File → Print Preview (Ctrl+P).

**Single source of truth.** Every option that's also on
`oxide_output::PdfOptions` lives ONLY on `pdf_options`; the
dispatcher mutates that struct directly so the rasterizer and
exporter see one consistent view. Fields on this struct itself are
the leftovers — UI presentation (active tab, quality enum), the
rasterised pages, and pan/zoom interaction state.

## Methods

- `pages`
- `page_handles`
- `selected`
- `pdf_options`
- `specific_page_input`
- `zoom`
- `active_tab`
- `pan`
- `panning`
- `sheet_files`
- `selected_files`
- `variants`
- `quality`

## Source
Lines 585–625 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
