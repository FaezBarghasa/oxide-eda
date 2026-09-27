---
okf_version: "0.2"
type: Module
title: pad
description: "Pads-mode Properties-panel surface: the pad form (types, message"
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/mod
language: rust
---

# pad

Pads-mode Properties-panel surface: the pad form (types, message

## Docstring

Pads-mode Properties-panel surface: the pad form (types, message
helpers, row primitives, and the three render functions), the pad-
stack preview + Choice enums, and the pad-properties table cells.

Folded from the former flat `pad_form` / `pad_stack_preview` /
`pad_table` siblings; `form` carries the cross-module surface the
parent panel and its sub-forms consume, re-exported below.
