---
okf_version: "0.2"
type: Module
title: qa_harness
description: QA harness — exercise every v0.8 exporter against a real Oxide
resource: crates/oxide-output/examples/qa_harness.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/examples/qa_harness
language: rust
---

# qa_harness

QA harness — exercise every v0.8 exporter against a real Oxide

## Docstring

QA harness — exercise every v0.8 exporter against a real Oxide
project and report sizes / sheet counts / validation issues.

Usage: `cargo run --example qa_harness -p oxide-output -- <project.snxprj> [out_dir]`

Reads the project, walks every sheet via the same logic the app uses,
drives PdfExporter / NetlistExporter / BomExporter (CSV / HTML / XLSX),
and prints a one-line summary per artefact plus any BOM validation
issues. Exits non-zero if any export panics or returns Err.

## Relationships

| Type | Target |
|------|--------|
| related | [main](/crates/oxide-output/examples/qa_harness/main.md) |
| related | [qa_pdf](/crates/oxide-output/examples/qa_harness/qa_pdf.md) |
| related | [qa_netlist](/crates/oxide-output/examples/qa_harness/qa_netlist.md) |
| related | [qa_bom](/crates/oxide-output/examples/qa_harness/qa_bom.md) |
| related | [truncate](/crates/oxide-output/examples/qa_harness/truncate.md) |
