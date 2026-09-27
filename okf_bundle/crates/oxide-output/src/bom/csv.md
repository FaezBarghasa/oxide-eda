---
okf_version: "0.2"
type: Module
title: csv
description: RFC 4180 CSV emitter for BOM export.
resource: crates/oxide-output/src/bom/csv.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/bom/csv
language: rust
---

# csv

RFC 4180 CSV emitter for BOM export.

## Docstring

RFC 4180 CSV emitter for BOM export.

Emits with UTF-8 BOM prefix and CRLF line endings for Windows compatibility.

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-output/src/bom/csv/emit.md) |
| related | [write_csv_row](/crates/oxide-output/src/bom/csv/write_csv_row.md) |
