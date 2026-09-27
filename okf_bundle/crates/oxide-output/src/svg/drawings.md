---
okf_version: "0.2"
type: Module
title: drawings
description: Free-standing schematic drawing emitter.
resource: crates/oxide-output/src/svg/drawings.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/drawings
language: rust
---

# drawings

Free-standing schematic drawing emitter.

## Docstring

Free-standing schematic drawing emitter.

Converts each `SchDrawing` (line / rect / polyline / circle / arc)
into `SvgElement` path primitives.

Extracted verbatim from the SVG exporter (`svg/mod.rs`); pure code
motion, zero behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [push_sch_drawing_path](/crates/oxide-output/src/svg/drawings/push_sch_drawing_path.md) |
