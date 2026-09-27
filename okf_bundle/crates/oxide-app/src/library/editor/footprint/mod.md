---
okf_version: "0.2"
type: Module
title: footprint
description: Footprint editor module — submodule re-exports only.
resource: crates/oxide-app/src/library/editor/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/mod
language: rust
---

# footprint

Footprint editor module — submodule re-exports only.

## Docstring

Footprint editor module — submodule re-exports only.

Footprint editing happens via the standalone `.snxfpt` document
editor in [`crate::library::editor::standalone`], which re-uses
[`canvas`] / [`layers`] / [`state`] verbatim.

`body3d`, `preview3d`, and `step_attach` remain on disk for the
eventual standalone Body 3D / STEP attach editor side-pane.
