# local_git_adapter

## Functions

- [cascade_footprint_personal_mode_auto_bumps_bound_row](cascade_footprint_personal_mode_auto_bumps_bound_row.md) — Footprint cascade mirrors symbol cascade — same predicate
- [cascade_personal_mode_auto_bumps_bound_row](cascade_personal_mode_auto_bumps_bound_row.md) — Personal-mode cascade silently bumps a row binding to the saved
- [cascade_team_mode_leaves_released_row_stale](cascade_team_mode_leaves_released_row_stale.md) — Team-mode + released row — cascade leaves the pinned
- [cascade_team_mode_unreleased_row_auto_bumps](cascade_team_mode_unreleased_row_auto_bumps.md) — Team-mode + unreleased row — auto-cascade fires (the released
- [corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid](corrupt_symbol_file_resolves_to_an_error_not_a_missing_uuid.md) — A corrupt `.snxsym` on disk must not read as "this row's symbol_ref
- [empty_snx_manifest](empty_snx_manifest.md)
- [fixture_footprint](fixture_footprint.md)
- [fixture_row](fixture_row.md) — Build a fixture row with a given internal PN and class. The `lib_id`
- [fixture_sim](fixture_sim.md)
- [fixture_symbol](fixture_symbol.md) — ── Primitive CRUD ───────────────────────────────────────────────────────
- [get_symbol_missing_uuid_is_not_found](get_symbol_missing_uuid_is_not_found.md) — [test]
- [history_returns_per_primitive_commits](history_returns_per_primitive_commits.md) — Stage 17: `history(primitive_path)` returns one [`HistoryEntry`]
- [init_adapter](init_adapter.md)
- [init_open_round_trip_empty_library](init_open_round_trip_empty_library.md) — Initialising at a non-existent path writes the `.snxlib` file + makes a
- [init_refuses_existing_library](init_refuses_existing_library.md) — Re-init over an existing `.snxlib` must not silently nuke history.
- [library_id_returns_manifest_id](library_id_returns_manifest_id.md) — `library_id()` reflects the manifest's stable UUID so the resolver can key
- [library_set_resolves_across_two_local_libs](library_set_resolves_across_two_local_libs.md) — LibrarySet integration test — mount two LocalGit libraries and resolve a
- [list_footprints_and_sims_are_name_sorted](list_footprints_and_sims_are_name_sorted.md) — `list_footprints` / `list_sims` read the name out of every
- [list_primitives_rejects_zero_primitive_envelope](list_primitives_rejects_zero_primitive_envelope.md) — A well-formed envelope carrying *zero* primitives is a corrupt
- [list_primitives_returns_alphabetic_summaries](list_primitives_returns_alphabetic_summaries.md) — `list_symbols` / `list_footprints` / `list_sims` walk the per-kind dir,
- [list_primitives_skips_non_uuid_and_foreign_files](list_primitives_skips_non_uuid_and_foreign_files.md) — Files whose stem is not a uuid, and files with a foreign extension,
- [local_git_commits_with_message](local_git_commits_with_message.md) — The `msg` argument on `insert_row` lands on the resulting libgit2 commit,
- [local_git_iter_rows_across_tables](local_git_iter_rows_across_tables.md) — `iter_rows` walks every `[tables.<name>]` inside the `.snxlib` and
- [local_git_read_row_by_pn](local_git_read_row_by_pn.md) — `read_row_by_pn` finds the first matching row across every table.
- [local_git_round_trip_row](local_git_round_trip_row.md) — `insert_row` → `read_row` round-trip; `update_row` mutates in place;
- [primitive_saves_each_create_a_commit](primitive_saves_each_create_a_commit.md) — Each `save_*` produces its own commit (so history mirrors edits).
- [save_then_get_footprint_round_trip](save_then_get_footprint_round_trip.md) — [test]
- [save_then_get_sim_round_trip](save_then_get_sim_round_trip.md) — [test]
- [save_then_get_symbol_round_trip](save_then_get_symbol_round_trip.md) — Save a Symbol → reopen → get_symbol → bytes are identical.
- [snx_manifest_with_mode](snx_manifest_with_mode.md) — Build an `.snxlib` manifest in the given workflow mode. Cascade
- [snxlib_path](snxlib_path.md) — Per-library .snxlib path inside `dir`. The file lives one level
