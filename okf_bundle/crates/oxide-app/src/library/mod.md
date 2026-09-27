---
okf_version: "0.2"
type: Module
title: library
description: "`oxide-app`'s library subsystem (DBLib model — v0.9-refactor-2)."
resource: crates/oxide-app/src/library/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mod
language: rust
---

# library

`oxide-app`'s library subsystem (DBLib model — v0.9-refactor-2).

## Docstring

`oxide-app`'s library subsystem (DBLib model — v0.9-refactor-2).

Wires the [`oxide_library`] crate (data + adapters) into the iced
UI. Components are TSV rows under `<lib>/tables/<category>.tsv`,
addressed by [`crate::library::state::EditorAddress`]. This module
owns:

* the **left-dock Library panel** — open libraries with inline
per-table row grids (see `panel.rs`);
* the **Place Component picker modal** (`picker.rs`);
* the **Component Preview tab** (`editor/`) — read-only Symbol +
Footprint render plus 5 tabs (Preview / Parameters / Supply /
Datasheet / Simulation). Symbol / Footprint editing happens via
the standalone `.snxsym` / `.snxfpt` document tabs;
* the **Distributor APIs settings** panel
(`settings/distributor_apis.rs`).
