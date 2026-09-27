---
okf_version: "0.2"
type: Module
title: properties_parameters
description: Properties panel — General + Parameters tabs (HI-22 / MD-20).
resource: crates/oxide-app/src/panels/properties_parameters/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/mod
language: rust
---

# properties_parameters

Properties panel — General + Parameters tabs (HI-22 / MD-20).

## Docstring

Properties panel — General + Parameters tabs (HI-22 / MD-20).

Extracted from `panels/mod.rs`. Pure view code, zero behaviour change.
Split into concern-sibling submodules:

- `general`   — the panel views (Custom Selection Filters, General +
Page Options, document-parameter table) and their button chrome.
- `form_rows` — generic form-field row builders shared by every
Properties surface.
- `net_params` — net-attribute rows, the Parameters (Net) section
chrome, and the justification-grid pickers.
