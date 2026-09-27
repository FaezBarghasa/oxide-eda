# regression

## Subdirectories

- [diagnostics_routing](diagnostics_routing/index.md)
- [dirty_paths_gateway](dirty_paths_gateway/index.md)
- [file_open_async](file_open_async/index.md)
- [history_load_failure](history_load_failure/index.md)
- [library_bga](library_bga/index.md)
- [library_browser_cell_commit](library_browser_cell_commit/index.md)
- [library_cross_track](library_cross_track/index.md)
- [library_pad_actions](library_pad_actions/index.md)
- [library_pad_geometry](library_pad_geometry/index.md)
- [library_placement](library_placement/index.md)
- [library_undo](library_undo/index.md)
- [preferences_dirty_guard](preferences_dirty_guard/index.md)
- [preferences_export_status](preferences_export_status/index.md)
- [preferences_prefs_recovery](preferences_prefs_recovery/index.md)
- [preferences_symbol_drafts](preferences_symbol_drafts/index.md)
- [prefs](prefs/index.md)
- [project](project/index.md)
- [render_config_grid_style](render_config_grid_style/index.md)
- [sketch_state](sketch_state/index.md)
- [undo_marker_divergence](undo_marker_divergence/index.md)

## Modules

- [diagnostics_routing](diagnostics_routing.md) — Records emitted while handling a message have to be on screen in the
- [dirty_paths_gateway](dirty_paths_gateway.md) — #585 — every engine edit must reach `dirty_paths`.
- [file_open_async](file_open_async.md) — #99 — schematic/PCB file-open read+parse is async.
- [history_load_failure](history_load_failure.md) — #599 — a git history walk that failed is not "no commits yet".
- [library_bga](library_bga.md) — BGA row/column numbering (skip-letters, start-row, start-col).
- [library_browser_cell_commit](library_browser_cell_commit.md) — #599 — an inline Library Browser cell commit must not retype a
- [library_cross_track](library_cross_track.md) — Phase-5 tests that span two tracks (undo + placement + geometry) at once — the Phase-5 counterparts of the Phase-3 `library_pad_geometry` tests.
- [library_pad_actions](library_pad_actions.md) — Pad selection, clipboard, rotate/flip, courtyard recompute, and context-menu dispatch.
- [library_pad_geometry](library_pad_geometry.md) — Parametric pad-shape mirror into sketch entities (round, round-rect, oval, chamfered pads).
- [library_placement](library_placement.md) — Sketch-tool gestures and live numeric placement input (typed distance/angle, Tab-cycling, Escape).
- [library_undo](library_undo.md) — `push_history` / `undo` / `redo` in isolation, no placement or geometry involved.
- [preferences_dirty_guard](preferences_dirty_guard.md) — Review #308 findings 1 + 2 — Preferences dirty-tracking must not miss an
- [preferences_export_status](preferences_export_status.md) — #533 Class C — a failed export write must reach the user.
- [preferences_prefs_recovery](preferences_prefs_recovery.md) — #602 — the Preferences prefs-file banner and its recovery action must
- [preferences_symbol_drafts](preferences_symbol_drafts.md) — #629 — the three Symbol Editor appearance settings must behave like
- [prefs](prefs.md) — `prefs.json` migration plus the read/write round-trip sweep.
- [project](project.md) — Project/document lifecycle: modals, git pipeline, exit guard, open-gating.
- [render_config_grid_style](render_config_grid_style.md) — #630 — the visible-grid style must reach the canvases as app state,
- [sketch_state](sketch_state.md) — Misfiled: these two exercise `oxide-sketch` state directly with no `Oxide`/`app.update` involved. True home is `crates/oxide-sketch/tests/`; kept here as-is per the split (see issue #432).
- [undo_marker_divergence](undo_marker_divergence.md) — #533 — the engine owns the undo history.
