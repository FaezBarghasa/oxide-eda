---
okf_version: "0.2"
type: Module
title: component_preview
description: "The Component Preview surface: inline editing of a library component"
resource: crates/oxide-app/src/library/component_preview/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/component_preview/mod
language: rust
---

# component_preview

The Component Preview surface: inline editing of a library component

## Docstring

The Component Preview surface: inline editing of a library component
row (datasheet, pin map, supply, parameters, simulation) without
opening the standalone primitive editors.

The update logic that applies these inline edits lives in [`updates`],
split by field-group concern.
