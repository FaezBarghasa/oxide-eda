---
okf_version: "0.2"
type: Module
title: footprint
description: "Footprint-editor dock message handlers, grouped by concern."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/mod
language: rust
---

# footprint

Footprint-editor dock message handlers, grouped by concern.

## Docstring

Footprint-editor dock message handlers, grouped by concern.

Each submodule holds the `impl Oxide` handlers for one slice of the
footprint-editor Properties/dock surface; the parent `sch_library`
router (`handle_dock_sch_library_message`) delegates each `PanelMsg`
arm to one of these. Split out of the former flat `footprint_*`
siblings so the shared `footprint` namespace lives in the folder, not
in every filename (ADR-0001 §5 — group by folder, no redundant prefix).
