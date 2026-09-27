---
okf_version: "0.2"
type: Module
title: state
description: In-memory state for the Library subsystem (DBLib model).
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod
language: rust
---

# state

In-memory state for the Library subsystem (DBLib model).

## Docstring

In-memory state for the Library subsystem (DBLib model).

Owned by [`crate::app::Oxide::library`]. In the v0.9-refactor-2
model, components are rows inside per-category TSV tables under
`<lib>/tables/<category>.tsv`, addressed by
`(library_path, table, row_id)`. The main pieces:

* `set` — `oxide_library::LibrarySet`, the cross-library resolver
that maps `library_id → Box<dyn LibraryAdapter>`. Editors and
renderers hand a `PrimitiveRef` to `set.resolve_*` to load
`Symbol`/`Footprint`/`SimModel` primitives without knowing which
library they live in.
* `open_libraries` — display caches per `*.snxlib/`. Each entry
holds the root path, display name, and per-table `Vec<ComponentRow>`
so the panel can render an inline grid per category without
re-reading disk between view ticks.
* `editors` — one entry per Component Preview tab keyed by
[`EditorAddress`]. The preview lives as a tab in the main
window's tab bar and may be undocked into its own OS window via
the standard tab-undock flow; either way the address is the
stable identity, not the window id.
* `picker` — component picker modal state (used by schematic
placement; flattens across every open library).
* `new_component` — modal state for the "New Row" flow
(library + table + class + InternalPN).
* `template_registry` — bundled + per-library parameter templates,
resolved at component-class lookup time.

## Relationships

| Type | Target |
|------|--------|
| related | [EditorAddress](/crates/oxide-app/src/library/state/mod/EditorAddress.md) |
| related | [new](/crates/oxide-app/src/library/state/mod/new.md) |
| related | [synthetic_tab_path](/crates/oxide-app/src/library/state/mod/synthetic_tab_path.md) |
| related | [new](/crates/oxide-app/src/library/state/mod/new.md) |
| related | [synthetic_tab_path](/crates/oxide-app/src/library/state/mod/synthetic_tab_path.md) |
| related | [LifecycleFilter](/crates/oxide-app/src/library/state/mod/LifecycleFilter.md) |
| related | [label](/crates/oxide-app/src/library/state/mod/label.md) |
| related | [allows](/crates/oxide-app/src/library/state/mod/allows.md) |
| related | [label](/crates/oxide-app/src/library/state/mod/label.md) |
| related | [allows](/crates/oxide-app/src/library/state/mod/allows.md) |
| related | [fmt](/crates/oxide-app/src/library/state/mod/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/state/mod/fmt.md) |
| related | [LibraryBrowserState](/crates/oxide-app/src/library/state/mod/LibraryBrowserState.md) |
| related | [NewClassDraft](/crates/oxide-app/src/library/state/mod/NewClassDraft.md) |
| related | [BrowserSort](/crates/oxide-app/src/library/state/mod/BrowserSort.md) |
| related | [new](/crates/oxide-app/src/library/state/mod/new.md) |
| related | [toggle_sort](/crates/oxide-app/src/library/state/mod/toggle_sort.md) |
| related | [new](/crates/oxide-app/src/library/state/mod/new.md) |
| related | [toggle_sort](/crates/oxide-app/src/library/state/mod/toggle_sort.md) |
| related | [DeleteConfirmState](/crates/oxide-app/src/library/state/mod/DeleteConfirmState.md) |
| related | [EditRowModalState](/crates/oxide-app/src/library/state/mod/EditRowModalState.md) |
| related | [new](/crates/oxide-app/src/library/state/mod/new.md) |
| related | [new](/crates/oxide-app/src/library/state/mod/new.md) |
| related | [PrimitivePickerState](/crates/oxide-app/src/library/state/mod/PrimitivePickerState.md) |
| related | [PrimitivePickerTarget](/crates/oxide-app/src/library/state/mod/PrimitivePickerTarget.md) |
| related | [LibraryState](/crates/oxide-app/src/library/state/mod/LibraryState.md) |
| related | [ComponentsMountSource](/crates/oxide-app/src/library/state/mod/ComponentsMountSource.md) |
| related | [label](/crates/oxide-app/src/library/state/mod/label.md) |
| related | [key](/crates/oxide-app/src/library/state/mod/key.md) |
| related | [label](/crates/oxide-app/src/library/state/mod/label.md) |
| related | [key](/crates/oxide-app/src/library/state/mod/key.md) |
| related | [ComponentsPanelState](/crates/oxide-app/src/library/state/mod/ComponentsPanelState.md) |
| related | [DocumentOptionsModalState](/crates/oxide-app/src/library/state/mod/DocumentOptionsModalState.md) |
| related | [LibraryCreateOptionsState](/crates/oxide-app/src/library/state/mod/LibraryCreateOptionsState.md) |
