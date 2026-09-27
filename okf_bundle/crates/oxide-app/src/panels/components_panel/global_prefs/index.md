# global_prefs

## Classs

- [GlobalLibraryEntry](GlobalLibraryEntry.md) — One row in `global_libraries.toml`. Mirrors the plan §3 schema —
- [GlobalPrefsFile](GlobalPrefsFile.md) — Top-level schema — `[[libraries]]` array.

## Functions

- [add_path](add_path.md) — Append a path to the global list and persist. Idempotent — already-
- [load](load.md) — Load the global library list from disk. Returns an empty Vec when
- [load_and_mount_all](load_and_mount_all.md) — One-shot "load and mount" — used by the bootstrap path so callers
- [load_at](load_at.md) — Load from a specific path — extracted so tests can hit the actual
- [load_returns_empty_when_file_missing](load_returns_empty_when_file_missing.md) — [test]
- [malformed_toml_returns_empty_via_load_path_logic](malformed_toml_returns_empty_via_load_path_logic.md) — [test]
- [mount_all](mount_all.md) — Mount every entry in `entries` onto the supplied `LibraryState`,
- [prefs_path](prefs_path.md) — Resolved on-disk path of `global_libraries.toml`. `None` when the
- [remove_path](remove_path.md) — Remove a path from the global list and persist.
- [round_trip_preserves_remote_and_auto_pull](round_trip_preserves_remote_and_auto_pull.md) — [test]
- [round_trip_serialises_path_only_entry](round_trip_serialises_path_only_entry.md) — [test]
- [save](save.md) — Persist `entries` to `global_libraries.toml`. Creates the parent
- [save_at](save_at.md) — Variant for tests / explicit paths — [`prefs_path`] is config-dir-global,
- [save_at_leaves_original_intact_when_write_fails](save_at_leaves_original_intact_when_write_fails.md) — `save_at` must go through `atomic_write`, not `fs::write`: a failed
