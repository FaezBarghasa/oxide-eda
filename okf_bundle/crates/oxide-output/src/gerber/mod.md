---
okf_version: "0.2"
type: Module
title: gerber
description: "Gerber RS-274X & Gerber X2/X3 Industrial Exporter for Oxide EDA."
resource: crates/oxide-output/src/gerber/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:02:33Z"
concept_id: crates/oxide-output/src/gerber/mod
language: rust
---

# gerber

Gerber RS-274X & Gerber X2/X3 Industrial Exporter for Oxide EDA.

## Docstring

Gerber RS-274X & Gerber X2/X3 Industrial Exporter for Oxide EDA.

Cleanroom implementation strictly adhering to Ucamco Gerber File Format Specification (Rev 2023.08).
Supports full multi-layer generation with standard apertures, aperture macros, X2 metadata attributes,
netlist attribution, and polygon area fills.

## Relationships

| Type | Target |
|------|--------|
| related | [GerberError](/crates/oxide-output/src/gerber/mod/GerberError.md) |
| related | [GerberLayer](/crates/oxide-output/src/gerber/mod/GerberLayer.md) |
| related | [file_extension](/crates/oxide-output/src/gerber/mod/file_extension.md) |
| related | [x2_file_function](/crates/oxide-output/src/gerber/mod/x2_file_function.md) |
| related | [file_extension](/crates/oxide-output/src/gerber/mod/file_extension.md) |
| related | [x2_file_function](/crates/oxide-output/src/gerber/mod/x2_file_function.md) |
| related | [ApertureDef](/crates/oxide-output/src/gerber/mod/ApertureDef.md) |
| related | [GerberOptions](/crates/oxide-output/src/gerber/mod/GerberOptions.md) |
| related | [default](/crates/oxide-output/src/gerber/mod/default.md) |
| related | [default](/crates/oxide-output/src/gerber/mod/default.md) |
| related | [GerberLayerOutput](/crates/oxide-output/src/gerber/mod/GerberLayerOutput.md) |
| related | [GerberExporter](/crates/oxide-output/src/gerber/mod/GerberExporter.md) |
| related | [new](/crates/oxide-output/src/gerber/mod/new.md) |
| related | [export_board](/crates/oxide-output/src/gerber/mod/export_board.md) |
| related | [export_single_layer](/crates/oxide-output/src/gerber/mod/export_single_layer.md) |
| related | [format_coord](/crates/oxide-output/src/gerber/mod/format_coord.md) |
| related | [new](/crates/oxide-output/src/gerber/mod/new.md) |
| related | [export_board](/crates/oxide-output/src/gerber/mod/export_board.md) |
| related | [export_single_layer](/crates/oxide-output/src/gerber/mod/export_single_layer.md) |
| related | [format_coord](/crates/oxide-output/src/gerber/mod/format_coord.md) |
| related | [test_gerber_export_headers_and_apertures](/crates/oxide-output/src/gerber/mod/test_gerber_export_headers_and_apertures.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
