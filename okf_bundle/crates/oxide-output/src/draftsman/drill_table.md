---
okf_version: "0.2"
type: Module
title: drill_table
description: "Automated Drill Table & Legend Synthesizer for Draftsman."
resource: crates/oxide-output/src/draftsman/drill_table.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:25:04Z"
concept_id: crates/oxide-output/src/draftsman/drill_table
language: rust
---

# drill_table

Automated Drill Table & Legend Synthesizer for Draftsman.

## Docstring

Automated Drill Table & Legend Synthesizer for Draftsman.

Conforms to Master Technical Directive §4.5:
- Live calculation and grouping of drill symbols by diameter, tolerance, plating condition, and hole count.
- Direct associativity with PCB layout database (pads, vias, mounting holes).

## Relationships

| Type | Target |
|------|--------|
| related | [HolePlating](/crates/oxide-output/src/draftsman/drill_table/HolePlating.md) |
| related | [DrillTableRow](/crates/oxide-output/src/draftsman/drill_table/DrillTableRow.md) |
| related | [DrillTable](/crates/oxide-output/src/draftsman/drill_table/DrillTable.md) |
| related | [from_board](/crates/oxide-output/src/draftsman/drill_table/from_board.md) |
| related | [from_board](/crates/oxide-output/src/draftsman/drill_table/from_board.md) |
| related | [test_drill_table_synthesis](/crates/oxide-output/src/draftsman/drill_table/test_drill_table_synthesis.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
