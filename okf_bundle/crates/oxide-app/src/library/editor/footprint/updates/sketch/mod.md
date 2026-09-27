---
okf_version: "0.2"
type: Module
title: sketch
description: Footprint sketch updates — the sketch concern folded into one
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/mod
language: rust
---

# sketch

Footprint sketch updates — the sketch concern folded into one

## Docstring

Footprint sketch updates — the sketch concern folded into one
folder, split by sub-concern (ADR-0001 D1/D2). The router
`updates::apply_footprint_primitive_edit` sends each sketch message
group to one of these modules' `apply`:

- [`ui`] — selection + tool/mode UI.
- [`placement`] — numeric placement-input buffer.
- [`entities`] — entity placement & drag geometry.
- [`pad_bridge`] — sketch↔pad bridge (roles / profile / corner radius).
- [`constraints`] — parameters & constraints.
- [`tools`] — the tool-click state machine (draw / edit / transform).

Each concern's `apply` was `pub(super)` on the former flat sibling
(visible to `updates`); moved one level deeper it widens to
`pub(in …updates)`, and the modules are exposed to the parent here
so the router's `sketch::<concern>::apply` call sites resolve.
