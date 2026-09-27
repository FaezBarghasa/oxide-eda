# mod

## Functions

- [add_arc_commit_stores_swapped_endpoints_for_a_cw_drag](add_arc_commit_stores_swapped_endpoints_for_a_cw_drag.md) — `AddArc` commits a CW-dragged placement (`end_deg < start_deg`)
- [apply_symbol_primitive_edit](apply_symbol_primitive_edit.md) — Apply a primitive-editor event to a standalone Symbol editor
- [arc_sweep_rejected_sets_status_message_without_committing](arc_sweep_rejected_sets_status_message_without_committing.md) — `ArcSweepRejected` surfaces a status message and commits
- [begin_drag_if_needed](begin_drag_if_needed.md) — Record the first event of a drag gesture.
- [clear_stale_status_message](clear_stale_status_message.md) — `SymbolEditorState::status_message`'s contract is "cleared on the
- [close_pickers](close_pickers.md) — Close any open colour picker (graphic-fill / local-colours). Call
- [collapse_consecutive_duplicate_vertices](collapse_consecutive_duplicate_vertices.md) — Collapse consecutive epsilon-duplicate points, including the
- [commit_or_discard_polygon](commit_or_discard_polygon.md) — Commit `editor.polygon_vertices` (the Place Polygon click-collect
- [context_submenu_msg_to_state](context_submenu_msg_to_state.md) — Translate the pure-data [`SymbolContextSubmenuMsg`] into the
- [context_target_msg_to_state](context_target_msg_to_state.md) — Translate the pure-data [`SymbolContextTargetMsg`] into the
- [dist_sq](dist_sq.md)
- [graphic_handle_msg_to_state](graphic_handle_msg_to_state.md) — Translate the pure-data [`GraphicHandleMsg`] back into the
- [mark_dirty](mark_dirty.md) — Mark the symbol as dirty and invalidate the canvas cache.
- [new_editor](new_editor.md)
- [normalize_arc_commit_deg](normalize_arc_commit_deg.md) — Normalise an about-to-be-stored `SymbolGraphicKind::Arc`'s
- [normalize_arc_commit_deg_leaves_ccw_pairs_unswapped](normalize_arc_commit_deg_leaves_ccw_pairs_unswapped.md) — An already-CCW (non-wrapped) pair is untouched beyond the
- [normalize_arc_commit_deg_swaps_a_cw_dragged_pair](normalize_arc_commit_deg_swaps_a_cw_dragged_pair.md) — `normalize_arc_commit_deg` swaps a CW-dragged pair (`end <
- [normalize_polygon_ring](normalize_polygon_ring.md) — Normalise a click-collected vertex ring before committing it:
- [polygon_cancel_discards_without_committing](polygon_cancel_discards_without_committing.md) — `PolygonCancel` discards the stash with no commit, regardless
- [polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry](polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry.md) — Three `PolygonClick`s then `PolygonCommit` push exactly one
- [polygon_commit_collapses_a_consecutive_duplicate_mid_sequence](polygon_commit_collapses_a_consecutive_duplicate_mid_sequence.md) — Two slow clicks landing on the same snapped point mid-sequence
- [polygon_commit_drops_duplicate_closing_vertex](polygon_commit_drops_duplicate_closing_vertex.md) — A trailing vertex equal to the first (a plain click landed
- [polygon_commit_with_collinear_vertices_is_discarded](polygon_commit_with_collinear_vertices_is_discarded.md) — A degenerate (collinear, zero-area) ring is discarded even
- [polygon_commit_with_fewer_than_three_vertices_is_discarded](polygon_commit_with_fewer_than_three_vertices_is_discarded.md) — Fewer than 3 collected vertices — `PolygonCommit` is a silent
- [polygon_commit_with_self_intersecting_bowtie_commits](polygon_commit_with_self_intersecting_bowtie_commits.md) — A self-intersecting bowtie whose crossed lobes cancel to
- [polygon_is_collinear](polygon_is_collinear.md) — `true` when every vertex in `points` lies within `eps` mm of the
- [push_graphic](push_graphic.md) — Push a graphic onto the symbol, recording an undo snapshot first.
- [push_undo](push_undo.md) — Push a full snapshot onto the undo stack; clear the redo stack.
- [push_undo_snapshot](push_undo_snapshot.md) — Push a pre-captured snapshot onto the undo stack and clear the redo
- [rotate_pivot_msg_to_state](rotate_pivot_msg_to_state.md) — Translate pure-data rotate pivot messages into Symbol-state pivot mode.
- [stale_status_message_clears_on_next_mutating_message](stale_status_message_clears_on_next_mutating_message.md) — A stale status message (e.g. left over from a failed
- [status_message_survives_camera_pan](status_message_survives_camera_pan.md) — Continuous/chrome-only messages (camera pan here) must NOT
- [symbol_bbox](symbol_bbox.md) — World-space bbox covering the symbol's body + every pin + every
