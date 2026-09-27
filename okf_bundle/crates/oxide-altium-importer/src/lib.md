---
okf_version: "0.2"
type: Module
title: lib
description: "Cleanroom pure-Rust Altium Designer (.SchDoc, .PcbDoc, .SchLib, .PcbLib, .IntLib) importer."
resource: crates/oxide-altium-importer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-altium-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:55:23Z"
concept_id: crates/oxide-altium-importer/src/lib
language: rust
---

# lib

Cleanroom pure-Rust Altium Designer (.SchDoc, .PcbDoc, .SchLib, .PcbLib, .IntLib) importer.

## Docstring

Cleanroom pure-Rust Altium Designer (.SchDoc, .PcbDoc, .SchLib, .PcbLib, .IntLib) importer.

# Overview
Reads and translates proprietary Altium OLE2/CFB binary streams into native
Oxide EDA types ([`SchematicSheet`], [`PcbBoard`], [`LibSymbol`], and [`Footprint`]).

## Relationships

| Type | Target |
|------|--------|
| related | [AltiumImportResult](/crates/oxide-altium-importer/src/lib/AltiumImportResult.md) |
| related | [import_altium_file](/crates/oxide-altium-importer/src/lib/import_altium_file.md) |
