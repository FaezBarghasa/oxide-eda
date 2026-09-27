---
okf_version: "0.2"
type: Module
title: excellon
description: Excellon NC Drill Exporter for Oxide EDA.
resource: crates/oxide-output/src/drill/excellon.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:33:05Z"
concept_id: crates/oxide-output/src/drill/excellon
language: rust
---

# excellon

Excellon NC Drill Exporter for Oxide EDA.

## Docstring

Excellon NC Drill Exporter for Oxide EDA.

Generates CNC drill files for PCB fabricators according to the Excellon 2 Format specification.
Supports Plated Through Holes (PTH), Non-Plated Holes (NPTH), tool table headers,
slot routing, and drill summary tables.

## Relationships

| Type | Target |
|------|--------|
| related | [DrillError](/crates/oxide-output/src/drill/excellon/DrillError.md) |
| related | [DrillHoleType](/crates/oxide-output/src/drill/excellon/DrillHoleType.md) |
| related | [DrillHole](/crates/oxide-output/src/drill/excellon/DrillHole.md) |
| related | [ExcellonOutput](/crates/oxide-output/src/drill/excellon/ExcellonOutput.md) |
| related | [ExcellonExporter](/crates/oxide-output/src/drill/excellon/ExcellonExporter.md) |
| related | [default](/crates/oxide-output/src/drill/excellon/default.md) |
| related | [default](/crates/oxide-output/src/drill/excellon/default.md) |
| related | [new](/crates/oxide-output/src/drill/excellon/new.md) |
| related | [export_board](/crates/oxide-output/src/drill/excellon/export_board.md) |
| related | [count_unique_tools](/crates/oxide-output/src/drill/excellon/count_unique_tools.md) |
| related | [generate_excellon_file](/crates/oxide-output/src/drill/excellon/generate_excellon_file.md) |
| related | [new](/crates/oxide-output/src/drill/excellon/new.md) |
| related | [export_board](/crates/oxide-output/src/drill/excellon/export_board.md) |
| related | [count_unique_tools](/crates/oxide-output/src/drill/excellon/count_unique_tools.md) |
| related | [generate_excellon_file](/crates/oxide-output/src/drill/excellon/generate_excellon_file.md) |
| related | [test_excellon_export_structure](/crates/oxide-output/src/drill/excellon/test_excellon_export_structure.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
