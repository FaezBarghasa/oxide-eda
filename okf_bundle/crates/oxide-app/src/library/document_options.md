---
okf_version: "0.2"
type: Module
title: document_options
description: Tools ▸ Document Options modal — Altium SchLib parity.
resource: crates/oxide-app/src/library/document_options.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/document_options
language: rust
---

# document_options

Tools ▸ Document Options modal — Altium SchLib parity.

## Docstring

Tools ▸ Document Options modal — Altium SchLib parity.

Per-`.snxlib` view settings: sheet color, grid spacing, grid
visibility, coordinate display unit. Edits go to a working
[`crate::library::state::DocumentOptionsModalState::draft`];
Save commits to `OpenLibrary.display`; Cancel discards.

Mounted as a full-screen overlay backdrop via
`app/view/mod.rs::collect_overlays` when
`LibraryState::document_options.is_some()`.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/document_options/view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
