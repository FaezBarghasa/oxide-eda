---
okf_version: "0.2"
type: Function
title: view_pdf_tab_strip
description: "Two-tab strip — Preview | Settings — sitting just under the"
resource: crates/oxide-app/src/app/view/pdf_preview/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/pdf_preview/mod/view_pdf_tab_strip
language: rust
---

# view_pdf_tab_strip

Two-tab strip — Preview | Settings — sitting just under the

## Signature

```rust
impl Oxide { pub(super) fn view_pdf_tab_strip(
        &self,
        active: crate::app::state::PdfPreviewTab,
    ) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Two-tab strip — Preview | Settings — sitting just under the
modal header. Uses the same `TabPill` widget the document tab
bar paints with: 3-sided border (top + L/R), accent stripe on
the active tab, fill that fades for inactive. `is_last=true`
on the rightmost so the trailing border doesn't double up
against an adjacent tab's left edge.

## Source
Lines 20–73 in `crates/oxide-app/src/app/view/pdf_preview/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdf_preview](/crates/oxide-app/src/app/view/pdf_preview/mod.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [pill_fill](/crates/oxide-app/src/tab_bar/pill_fill.md) |
