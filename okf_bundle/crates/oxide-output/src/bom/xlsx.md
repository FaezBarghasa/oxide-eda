---
okf_version: "0.2"
type: Module
title: xlsx
description: XLSX emitter for BOM export via rust_xlsxwriter.
resource: crates/oxide-output/src/bom/xlsx.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/bom/xlsx
language: rust
---

# xlsx

XLSX emitter for BOM export via rust_xlsxwriter.

## Docstring

XLSX emitter for BOM export via rust_xlsxwriter.

Produces an Excel workbook with frozen header row, auto-fit column widths,
and styled header row (bold, grey background).

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-output/src/bom/xlsx/emit.md) |
| related | [rust_xlsxwriter](/_dependencies/cargo/rust_xlsxwriter.md) |
