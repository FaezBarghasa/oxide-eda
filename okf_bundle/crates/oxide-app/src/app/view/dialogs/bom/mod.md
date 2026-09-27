---
okf_version: "0.2"
type: Module
title: bom
description: "BOM (Bill of Materials) preview modal — the modal shell (header, toolbar"
resource: crates/oxide-app/src/app/view/dialogs/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/mod
language: rust
---

# bom

BOM (Bill of Materials) preview modal — the modal shell (header, toolbar

## Docstring

BOM (Bill of Materials) preview modal — the modal shell (header, toolbar
strip, footer) that stitches together the data grid (`table`) and the
Properties sidebar (`sidebar`).

Extracted from `view/dialogs.rs` (ADR-0001, issue #164). The section
builders were split into `bom/table.rs` and `bom/sidebar.rs` as pure code
motion — the final `column!`/`row!` child order is preserved byte-for-byte,
so the rendered modal is pixel-identical.

## Relationships

| Type | Target |
|------|--------|
| related | [view_bom_preview](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview.md) |
| related | [view_bom_preview_body](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body.md) |
| related | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| related | [view_bom_preview](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview.md) |
| related | [view_bom_preview_body](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body.md) |
| related | [view_bom_preview_body_inner](/crates/oxide-app/src/app/view/dialogs/bom/mod/view_bom_preview_body_inner.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
