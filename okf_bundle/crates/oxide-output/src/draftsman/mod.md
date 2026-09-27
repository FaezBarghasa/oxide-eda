---
okf_version: "0.2"
type: Module
title: draftsman
description: "`draftsman` — Live Associative Production Engineering & Drafting Engine."
resource: crates/oxide-output/src/draftsman/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:25:28Z"
concept_id: crates/oxide-output/src/draftsman/mod
language: rust
---

# draftsman

`draftsman` — Live Associative Production Engineering & Drafting Engine.

## Docstring

`draftsman` — Live Associative Production Engineering & Drafting Engine.

Conforms to Master Technical Directive §4.5:
- Bidirectional Associative Model: Drawing views (Fabrication Views, Assembly Views, Drill Legends, Stackup Drawings)
maintain direct live-linked DAG dependencies with the core PCB database.
- Standards Compliance: ASME Y14.5 and ISO 128 Geometric Dimensioning and Tolerancing (GD&T).
- Automated Table Synthesis: Real-time grouping of drill symbols, tolerances, and plating conditions.

## Relationships

| Type | Target |
|------|--------|
| related | [SheetSize](/crates/oxide-output/src/draftsman/mod/SheetSize.md) |
| related | [dimensions_mm](/crates/oxide-output/src/draftsman/mod/dimensions_mm.md) |
| related | [dimensions_mm](/crates/oxide-output/src/draftsman/mod/dimensions_mm.md) |
| related | [DrawingView](/crates/oxide-output/src/draftsman/mod/DrawingView.md) |
| related | [DraftsmanSheet](/crates/oxide-output/src/draftsman/mod/DraftsmanSheet.md) |
| related | [new](/crates/oxide-output/src/draftsman/mod/new.md) |
| related | [new](/crates/oxide-output/src/draftsman/mod/new.md) |
| related | [DraftsmanDocument](/crates/oxide-output/src/draftsman/mod/DraftsmanDocument.md) |
| related | [new](/crates/oxide-output/src/draftsman/mod/new.md) |
| related | [sync_with_board](/crates/oxide-output/src/draftsman/mod/sync_with_board.md) |
| related | [new](/crates/oxide-output/src/draftsman/mod/new.md) |
| related | [sync_with_board](/crates/oxide-output/src/draftsman/mod/sync_with_board.md) |
| related | [test_draftsman_document_creation_and_sync](/crates/oxide-output/src/draftsman/mod/test_draftsman_document_creation_and_sync.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
