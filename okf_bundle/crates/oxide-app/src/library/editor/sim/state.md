---
okf_version: "0.2"
type: Module
title: state
description: Sim tab state.
resource: crates/oxide-app/src/library/editor/sim/state.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/sim/state
language: rust
---

# state

Sim tab state.

## Docstring

Sim tab state.

All persistent state for the Sim tab lives on the typed
[`oxide_library::SimModel`] primitive bound through
`ComponentRow::sim_ref`. The only piece of UI-only state — the
live `text_editor::Content` mirror of the SPICE deck — sits on
[`crate::library::state::ComponentPreviewState::sim_body`] so it
shares the per-editor lifetime that the rest of the tab tooling
uses.

This module stays as a no-op surface: dropping the file would
cascade into `pub mod state;` removals across the editor module
tree; the empty `SimTabState` keeps the public surface stable
for any sibling that imports it.

## Relationships

| Type | Target |
|------|--------|
| related | [SimTabState](/crates/oxide-app/src/library/editor/sim/state/SimTabState.md) |
