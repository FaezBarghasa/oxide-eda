---
okf_version: "0.2"
type: Module
title: selected
description: Properties surface for a single selected schematic element.
resource: crates/oxide-app/src/panels/element_properties/selected.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/element_properties/selected
language: rust
---

# selected

Properties surface for a single selected schematic element.

## Docstring

Properties surface for a single selected schematic element.

Routes between Symbol / Reference-or-Value field / Label / TextNote
/ Drawing / ChildSheet contexts. Moved verbatim from the former
single-file `element_properties` module — pure view code, zero
behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [view_selected_element_properties](/crates/oxide-app/src/panels/element_properties/selected/view_selected_element_properties.md) |
