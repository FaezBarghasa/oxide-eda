---
okf_version: "0.2"
type: Module
title: parameters
description: Parameter edits for a Component Preview row.
resource: crates/oxide-app/src/library/component_preview/updates/parameters.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/parameters
language: rust
---

# parameters

Parameter edits for a Component Preview row.

## Docstring

Parameter edits for a Component Preview row.

Text and boolean parameters commit immediately; numeric and
measurement parameters are edited through a per-row buffer
(`params_edit_buf`) and committed on demand so a half-typed value is
never parsed.

## Relationships

| Type | Target |
|------|--------|
| related | [set_text](/crates/oxide-app/src/library/component_preview/updates/parameters/set_text.md) |
| related | [set_number_buf](/crates/oxide-app/src/library/component_preview/updates/parameters/set_number_buf.md) |
| related | [commit_number](/crates/oxide-app/src/library/component_preview/updates/parameters/commit_number.md) |
| related | [set_measurement_buf](/crates/oxide-app/src/library/component_preview/updates/parameters/set_measurement_buf.md) |
| related | [commit_measurement](/crates/oxide-app/src/library/component_preview/updates/parameters/commit_measurement.md) |
| related | [set_bool](/crates/oxide-app/src/library/component_preview/updates/parameters/set_bool.md) |
| related | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| related | [add_custom](/crates/oxide-app/src/library/component_preview/updates/parameters/add_custom.md) |
