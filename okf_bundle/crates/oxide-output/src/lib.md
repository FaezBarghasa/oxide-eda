---
okf_version: "0.2"
type: Module
title: lib
description: "Output generation for Oxide — PDF, BOM, netlist."
resource: crates/oxide-output/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:20:47Z"
concept_id: crates/oxide-output/src/lib
language: rust
---

# lib

Output generation for Oxide — PDF, BOM, netlist.

## Docstring

Output generation for Oxide — PDF, BOM, netlist.

See `docs/internal/docs/OUTPUT_PLAN.md` for the v0.8 design.

v0.8.0 ships: PDF, netlist, print preview, sheet templates, text substitution.
v0.8.1 ships: BOM (CSV / HTML / XLSX).

## Relationships

| Type | Target |
|------|--------|
| related | [Exporter](/crates/oxide-output/src/lib/Exporter.md) |
| related | [ExportContext](/crates/oxide-output/src/lib/ExportContext.md) |
| related | [SheetSnapshot](/crates/oxide-output/src/lib/SheetSnapshot.md) |
| related | [ProjectMetadata](/crates/oxide-output/src/lib/ProjectMetadata.md) |
| related | [ExportError](/crates/oxide-output/src/lib/ExportError.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
