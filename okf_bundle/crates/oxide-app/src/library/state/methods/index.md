# methods

## Classs

- [LibraryDisplaySettings](LibraryDisplaySettings.md) — Per-library canvas + UI defaults shared across every primitive
- [OpenLibrary](OpenLibrary.md) — One open `*.snxlib/` directory — display cache only. The owning

## Functions

- [all_components](all_components.md) — Aggregate every open library's cached components — used by the
- [all_components](all_components_1.md) — Aggregate every open library's cached components — used by the
- [apply_primitive_listing](apply_primitive_listing.md) — Move one primitive listing onto its cache, keeping the previous
- [close_library](close_library.md) — Drop the library backing `root` — unmounts from `set` and drops
- [close_library](close_library_1.md) — Drop the library backing `root` — unmounts from `set` and drops
- [containing_library](containing_library.md) — Find the open library whose `root_dir` is an ancestor of
- [containing_library](containing_library_1.md) — Find the open library whose `root_dir` is an ancestor of
- [containing_library_mut](containing_library_mut.md)
- [containing_library_mut](containing_library_mut_1.md)
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [dirty_editors_for_library](dirty_editors_for_library.md) — Editor addresses currently pointing at `root` that have unsaved edits.
- [dirty_editors_for_library](dirty_editors_for_library_1.md) — Editor addresses currently pointing at `root` that have unsaved edits.
- [editor_for](editor_for.md) — Existing editor for `(library_root, table, row_id)`, if any.
- [editor_for](editor_for_1.md) — Existing editor for `(library_root, table, row_id)`, if any.
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [ingest_sheet](ingest_sheet.md) — Replace the Where-Used entries for one `(project, sheet)` with
- [ingest_sheet](ingest_sheet_1.md) — Replace the Where-Used entries for one `(project, sheet)` with
- [library_at](library_at.md) — Look up an open library by its on-disk root path.
- [library_at](library_at_1.md) — Look up an open library by its on-disk root path.
- [library_at_mut](library_at_mut.md)
- [library_at_mut](library_at_mut_1.md)
- [mount_source_for](mount_source_for.md) — Classify how a mounted library got there — drives the
- [mount_source_for](mount_source_for_1.md) — Classify how a mounted library got there — drives the
- [open_library](open_library.md) — Open the `*.snxlib/` at `root`, mounting the adapter under its
- [open_library](open_library_1.md) — Open the `*.snxlib/` at `root`, mounting the adapter under its
- [refresh_components](refresh_components.md) — Refresh the cached table contents for a library — re-reads every
- [refresh_components](refresh_components_1.md) — Refresh the cached table contents for a library — re-reads every
- [reload_primitives](reload_primitives.md) — Refresh the cached `(symbols, footprints, sims)` summary
- [reload_primitives](reload_primitives_1.md) — Refresh the cached `(symbols, footprints, sims)` summary
- [reload_tables](reload_tables.md) — Re-read every TSV via the supplied adapter. Replaces both
- [reload_tables](reload_tables_1.md) — Re-read every TSV via the supplied adapter. Replaces both
- [root_dir](root_dir.md) — Directory holding the `.snxlib` file — the per-library git
- [root_dir](root_dir_1.md) — Directory holding the `.snxlib` file — the per-library git
- [total_rows](total_rows.md) — Total number of rows across every cached table.
- [total_rows](total_rows_1.md) — Total number of rows across every cached table.
- [where_used_for](where_used_for.md) — Look up the use-sites for a row.
- [where_used_for](where_used_for_1.md) — Look up the use-sites for a row.
