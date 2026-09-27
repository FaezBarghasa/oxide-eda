# adapter

## Classs

- [ComponentSummary](ComponentSummary.md) — One result row from a library query — header info derived from a
- [FieldSet](FieldSet.md) — Field-sets per v0.9-library-plan.md §8 — locking granularity.
- [HistoryEntry](HistoryEntry.md) — One entry in the per-primitive git history feed.
- [LibraryAdapter](LibraryAdapter.md) — Storage backend abstraction. All flavours (LocalGit, Database, Plm)
- [LibraryError](LibraryError.md) — [derive(Debug, thiserror::Error)]
- [LibraryQuery](LibraryQuery.md) — A query into the library — partial match on internal_pn or mpn, plus facets.
- [PrimitiveSummary](PrimitiveSummary.md) — Header row for a primitive listing — name + uuid + kind tag, plus a hint

## Functions

- [_accepts_dyn](accepts_dyn.md) — Compile-time check — if this compiles, the trait is dyn-compatible.
- [add_library_class](add_library_class.md) — Atomic helper — append a class. The default implementation
- [commit_external_change](commit_external_change.md) — Stage and commit a file the caller already wrote to disk, using
- [create_empty_table](create_empty_table.md) — Create an empty table named `name`. Used by the New Component
- [delete_empty_table](delete_empty_table.md) — Delete the table `name` from the library. Adapters MUST refuse
- [delete_row](delete_row.md)
- [get_footprint](get_footprint.md)
- [get_sim](get_sim.md)
- [get_symbol](get_symbol.md) — ── Primitive CRUD (unchanged from v0.9-original) ───────────────────
- [history](history.md) — Per-primitive git history.
- [insert_row](insert_row.md)
- [iter_rows](iter_rows.md) — Iterate every row across every table — `(table_name, row)` pairs.
- [library_adapter_is_object_safe](library_adapter_is_object_safe.md) — [test]
- [library_classes](library_classes.md) — Per-library class registry — the source of truth for the New
- [library_file](library_file.md) — Borrow the parsed `.snxlib` view if this adapter is backed by an
- [library_file_path](library_file_path.md) — Absolute path to the `.snxlib` file itself. `None` for non-file
- [library_id](library_id.md) — Stable UUID of this library, sourced from `library.toml::library.library_id`.
- [library_query_default_is_empty](library_query_default_is_empty.md) — [test]
- [list_footprints](list_footprints.md)
- [list_sims](list_sims.md)
- [list_symbols](list_symbols.md)
- [list_tables](list_tables.md) — List the names of every table this library exposes (filename stem,
- [read_row](read_row.md) — ── Row CRUD ────────────────────────────────────────────────────────
- [read_row_by_pn](read_row_by_pn.md)
- [read_table](read_table.md) — Read every row from the named table.
- [release_lock](release_lock.md)
- [remove_library_class](remove_library_class.md) — Atomic helper — remove a class by key. Default goes through
- [rename_library_class](rename_library_class.md) — Atomic helper — rename a class. `old_key` must currently
- [rename_table](rename_table.md) — Rename a table — `old` → `new`, preserving every row inside.
- [root_dir](root_dir.md) — Absolute path to the directory that *contains* the `.snxlib` file
- [root_path](root_path.md) — For local-git, the `.snxlib/` directory; for DB, `None`.
- [save_footprint](save_footprint.md)
- [save_sim](save_sim.md)
- [save_symbol](save_symbol.md)
- [try_lock](try_lock.md) — ── Locks (advisory) ────────────────────────────────────────────────
- [update_library_classes](update_library_classes.md) — Persist the class registry. The UI calls this from the
- [update_row](update_row.md)
