---
okf_version: "0.2"
type: Module
title: sim
description: Simulation-model edits for a Component Preview row.
resource: crates/oxide-app/src/library/component_preview/updates/sim.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/sim
language: rust
---

# sim

Simulation-model edits for a Component Preview row.

## Docstring

Simulation-model edits for a Component Preview row.

Enabling simulation mints a fresh SPICE `SimModel` and binds it to the
row; the remaining actions edit the model's kind, name, body text, and
per-pin node map while it exists.

## Relationships

| Type | Target |
|------|--------|
| related | [set_enabled](/crates/oxide-app/src/library/component_preview/updates/sim/set_enabled.md) |
| related | [set_kind](/crates/oxide-app/src/library/component_preview/updates/sim/set_kind.md) |
| related | [set_name](/crates/oxide-app/src/library/component_preview/updates/sim/set_name.md) |
| related | [apply_body_action](/crates/oxide-app/src/library/component_preview/updates/sim/apply_body_action.md) |
| related | [set_pin_node](/crates/oxide-app/src/library/component_preview/updates/sim/set_pin_node.md) |
