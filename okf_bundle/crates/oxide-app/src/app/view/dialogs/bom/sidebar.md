---
okf_version: "0.2"
type: Module
title: sidebar
description: BOM preview — the Properties sidebar (General / Columns tabs) plus the
resource: crates/oxide-app/src/app/view/dialogs/bom/sidebar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/bom/sidebar
language: rust
---

# sidebar

BOM preview — the Properties sidebar (General / Columns tabs) plus the

## Docstring

BOM preview — the Properties sidebar (General / Columns tabs) plus the
grouping / format / toggle / variant / column control rows it consumes.
Extracted from `dialogs/bom.rs` (ADR-0001, issue #164) as pure code
motion — the child-push order inside every row/column is preserved
byte-for-byte, so the rendered sidebar is pixel-identical.

## Relationships

| Type | Target |
|------|--------|
| related | [bom_sidebar](/crates/oxide-app/src/app/view/dialogs/bom/sidebar/bom_sidebar.md) |
| related | [bom_sidebar](/crates/oxide-app/src/app/view/dialogs/bom/sidebar/bom_sidebar.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
