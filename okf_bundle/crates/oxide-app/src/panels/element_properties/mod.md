---
okf_version: "0.2"
type: Module
title: element_properties
description: Properties panel for a single selected schematic element (HI-22 / MD-20).
resource: crates/oxide-app/src/panels/element_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/element_properties/mod
language: rust
---

# element_properties

Properties panel for a single selected schematic element (HI-22 / MD-20).

## Docstring

Properties panel for a single selected schematic element (HI-22 / MD-20).

Extracted from `panels/mod.rs`. Pure view code — zero behaviour change.
Routes between Symbol / Label / TextNote / Drawing / ChildSheet
contexts; the per-shape Drawing surface and per-child-sheet style
editor live in sibling submodules split out of the former
single-file module.
