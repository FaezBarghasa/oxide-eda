---
okf_version: "0.2"
type: Module
title: constraints
description: "Footprint sketch updates — parameters & constraints concern."
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints
language: rust
---

# constraints

Footprint sketch updates — parameters & constraints concern.

## Docstring

Footprint sketch updates — parameters & constraints concern.

Carved out of the monolithic `sketch::apply` (ADR-0001 D1/D2). `apply`
is a thin router; each variant delegates to one named per-action fn
below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/apply.md) |
| related | [edit_parameter](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/edit_parameter.md) |
| related | [add_constraint_for_selection](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection.md) |
| related | [dim_input_error](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/dim_input_error.md) |
| related | [report_constraint_not_added](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/report_constraint_not_added.md) |
