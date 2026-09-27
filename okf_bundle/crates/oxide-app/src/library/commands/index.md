# commands

## Classs

- [AutoMountOutcome](AutoMountOutcome.md) — What one `auto_mount_project_libraries` pass decided — issue #99
- [PendingLibrarySpec](PendingLibrarySpec.md) — Captured shape of a New Library request that hasn't been written

## Functions

- [auto_mount_project_libraries](auto_mount_project_libraries.md) — Auto-mount every library referenced by `project.libraries`. Called
- [create_component_row](create_component_row.md) — Create a new component **row**:
- [create_library](create_library.md) — Convenience wrapper — create a project-local library named
- [create_library_at](create_library_at.md) — Create a fresh `.snxlib/` library at `lib_path`. The directory's
- [jump_to_use_site](jump_to_use_site.md) — Stub: emit `tracing::info!` with the use-site coordinates the
- [list_components_filtered](list_components_filtered.md) — Re-run a query against every open library — picker filter helper.
- [materialize_pending_library](materialize_pending_library.md) — Materialise a previously-registered pending library: do the
- [open_library](open_library.md) — Open a `*.snxlib/` and, when it was already mounted, refresh its
- [register_pending_library](register_pending_library.md) — Register a library creation request without touching disk.
