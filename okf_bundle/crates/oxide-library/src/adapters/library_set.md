---
okf_version: "0.2"
type: Module
title: library_set
description: "`LibrarySet` — a tiny resolver that composes any number of"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set
language: rust
---

# library_set

`LibrarySet` — a tiny resolver that composes any number of

## Docstring

`LibrarySet` — a tiny resolver that composes any number of
[`LibraryAdapter`] trait objects into a single lookup surface for
cross-library [`PrimitiveRef`] resolution.

Per `v0.9-snxlib-as-file-plan.md` §2 Stage B, the primary mount
key is the `.snxlib` *file path* (not the library_id). The
Components Panel (§5) presents the same library_id across
Project / Installed / Global lists — a library_id can therefore
legitimately appear under multiple file paths in a single
`LibrarySet`, so the duplicate-id check that used to gate
mounting has moved out to the panel layer (where dedup happens
at presentation time).

## Resolution semantics

The resolvers return `Result<Option<T>, LibraryError>` because
"the primitive is not there" and "the lookup could not be
performed" are different answers and the user needs a different
message for each:

- `Ok(None)` — the reference genuinely does not resolve. Either the
`library_id` isn't mounted (the editor surfaces this as
"unresolved primitive — open dependent library?") or the
`library_id` IS mounted but the primitive UUID isn't in any of
the matching adapters.
- `Err(_)` — the adapter could not tell: I/O on `symbols/`, a
corrupt `.snxsym` / `.snxfpt` on disk, or an HTTP failure from
`DatabaseAdapter` talking to a remote library server. Collapsing
these into `Ok(None)` reports a perfectly good binding as a wrong
one, and sends the user off to re-bind a valid UUID or to hunt a
phantom mount problem.

Only [`LibraryError::NotFound`] is evidence of absence; every other
variant is a failure to look.

- When two adapters share a `library_id` (the "user copy-pasted a
library" case), [`LibrarySet::resolve_symbol`] / `_footprint` /
`_sim` pick the first match. UI dedup at the Components Panel
level is the right place to warn the user about the collision.
- Adapters without an on-disk path (e.g. `DatabaseAdapter`) fall
back to keying by `library_id`; the duplicate-id error still
guards those.

## Relationships

| Type | Target |
|------|--------|
| related | [MountKey](/crates/oxide-library/src/adapters/library_set/MountKey.md) |
| related | [for_adapter](/crates/oxide-library/src/adapters/library_set/for_adapter.md) |
| related | [for_adapter](/crates/oxide-library/src/adapters/library_set/for_adapter.md) |
| related | [LibrarySet](/crates/oxide-library/src/adapters/library_set/LibrarySet.md) |
| related | [new](/crates/oxide-library/src/adapters/library_set/new.md) |
| related | [mount](/crates/oxide-library/src/adapters/library_set/mount.md) |
| related | [remount](/crates/oxide-library/src/adapters/library_set/remount.md) |
| related | [unmount](/crates/oxide-library/src/adapters/library_set/unmount.md) |
| related | [unmount_by_path](/crates/oxide-library/src/adapters/library_set/unmount_by_path.md) |
| related | [len](/crates/oxide-library/src/adapters/library_set/len.md) |
| related | [is_empty](/crates/oxide-library/src/adapters/library_set/is_empty.md) |
| related | [contains](/crates/oxide-library/src/adapters/library_set/contains.md) |
| related | [contains_path](/crates/oxide-library/src/adapters/library_set/contains_path.md) |
| related | [get](/crates/oxide-library/src/adapters/library_set/get.md) |
| related | [get_by_path](/crates/oxide-library/src/adapters/library_set/get_by_path.md) |
| related | [library_ids](/crates/oxide-library/src/adapters/library_set/library_ids.md) |
| related | [library_paths](/crates/oxide-library/src/adapters/library_set/library_paths.md) |
| related | [resolve_symbol](/crates/oxide-library/src/adapters/library_set/resolve_symbol.md) |
| related | [resolve_footprint](/crates/oxide-library/src/adapters/library_set/resolve_footprint.md) |
| related | [resolve_sim](/crates/oxide-library/src/adapters/library_set/resolve_sim.md) |
| related | [unresolved_refs](/crates/oxide-library/src/adapters/library_set/unresolved_refs.md) |
| related | [find_adapter_for_id](/crates/oxide-library/src/adapters/library_set/find_adapter_for_id.md) |
| related | [find_key_for_id](/crates/oxide-library/src/adapters/library_set/find_key_for_id.md) |
| related | [new](/crates/oxide-library/src/adapters/library_set/new.md) |
| related | [mount](/crates/oxide-library/src/adapters/library_set/mount.md) |
| related | [remount](/crates/oxide-library/src/adapters/library_set/remount.md) |
| related | [unmount](/crates/oxide-library/src/adapters/library_set/unmount.md) |
| related | [unmount_by_path](/crates/oxide-library/src/adapters/library_set/unmount_by_path.md) |
| related | [len](/crates/oxide-library/src/adapters/library_set/len.md) |
| related | [is_empty](/crates/oxide-library/src/adapters/library_set/is_empty.md) |
| related | [contains](/crates/oxide-library/src/adapters/library_set/contains.md) |
| related | [contains_path](/crates/oxide-library/src/adapters/library_set/contains_path.md) |
| related | [get](/crates/oxide-library/src/adapters/library_set/get.md) |
| related | [get_by_path](/crates/oxide-library/src/adapters/library_set/get_by_path.md) |
| related | [library_ids](/crates/oxide-library/src/adapters/library_set/library_ids.md) |
| related | [library_paths](/crates/oxide-library/src/adapters/library_set/library_paths.md) |
| related | [resolve_symbol](/crates/oxide-library/src/adapters/library_set/resolve_symbol.md) |
| related | [resolve_footprint](/crates/oxide-library/src/adapters/library_set/resolve_footprint.md) |
| related | [resolve_sim](/crates/oxide-library/src/adapters/library_set/resolve_sim.md) |
| related | [unresolved_refs](/crates/oxide-library/src/adapters/library_set/unresolved_refs.md) |
| related | [find_adapter_for_id](/crates/oxide-library/src/adapters/library_set/find_adapter_for_id.md) |
| related | [find_key_for_id](/crates/oxide-library/src/adapters/library_set/find_key_for_id.md) |
| related | [UnresolvedRefs](/crates/oxide-library/src/adapters/library_set/UnresolvedRefs.md) |
| related | [is_empty](/crates/oxide-library/src/adapters/library_set/is_empty.md) |
| related | [is_empty](/crates/oxide-library/src/adapters/library_set/is_empty.md) |
| related | [Probe](/crates/oxide-library/src/adapters/library_set/Probe.md) |
| related | [probe_every_kind](/crates/oxide-library/src/adapters/library_set/probe_every_kind.md) |
| related | [absence_is_none](/crates/oxide-library/src/adapters/library_set/absence_is_none.md) |
| related | [fmt](/crates/oxide-library/src/adapters/library_set/fmt.md) |
| related | [fmt](/crates/oxide-library/src/adapters/library_set/fmt.md) |
| related | [FakeAdapter](/crates/oxide-library/src/adapters/library_set/FakeAdapter.md) |
| related | [new](/crates/oxide-library/src/adapters/library_set/new.md) |
| related | [with_symbol](/crates/oxide-library/src/adapters/library_set/with_symbol.md) |
| related | [with_path](/crates/oxide-library/src/adapters/library_set/with_path.md) |
| related | [with_read_failure](/crates/oxide-library/src/adapters/library_set/with_read_failure.md) |
| related | [failure](/crates/oxide-library/src/adapters/library_set/failure.md) |
| related | [new](/crates/oxide-library/src/adapters/library_set/new.md) |
| related | [with_symbol](/crates/oxide-library/src/adapters/library_set/with_symbol.md) |
| related | [with_path](/crates/oxide-library/src/adapters/library_set/with_path.md) |
| related | [with_read_failure](/crates/oxide-library/src/adapters/library_set/with_read_failure.md) |
| related | [failure](/crates/oxide-library/src/adapters/library_set/failure.md) |
| related | [manifest](/crates/oxide-library/src/adapters/library_set/manifest.md) |
| related | [library_file_path](/crates/oxide-library/src/adapters/library_set/library_file_path.md) |
| related | [get_symbol](/crates/oxide-library/src/adapters/library_set/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapters/library_set/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapters/library_set/get_sim.md) |
| related | [manifest](/crates/oxide-library/src/adapters/library_set/manifest.md) |
| related | [library_file_path](/crates/oxide-library/src/adapters/library_set/library_file_path.md) |
| related | [get_symbol](/crates/oxide-library/src/adapters/library_set/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapters/library_set/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapters/library_set/get_sim.md) |
| related | [fixture_symbol](/crates/oxide-library/src/adapters/library_set/fixture_symbol.md) |
| related | [empty_set_resolves_nothing](/crates/oxide-library/src/adapters/library_set/empty_set_resolves_nothing.md) |
| related | [mount_and_resolve_symbol](/crates/oxide-library/src/adapters/library_set/mount_and_resolve_symbol.md) |
| related | [unresolved_when_library_missing](/crates/oxide-library/src/adapters/library_set/unresolved_when_library_missing.md) |
| related | [unresolved_when_uuid_missing_in_mounted_lib](/crates/oxide-library/src/adapters/library_set/unresolved_when_uuid_missing_in_mounted_lib.md) |
| related | [read_failure_is_reported_not_reported_as_a_missing_uuid](/crates/oxide-library/src/adapters/library_set/read_failure_is_reported_not_reported_as_a_missing_uuid.md) |
| related | [unresolved_refs_filters_to_only_missing](/crates/oxide-library/src/adapters/library_set/unresolved_refs_filters_to_only_missing.md) |
| related | [unresolved_refs_separates_unreadable_from_missing](/crates/oxide-library/src/adapters/library_set/unresolved_refs_separates_unreadable_from_missing.md) |
| related | [unmount_returns_adapter_and_drops_resolution](/crates/oxide-library/src/adapters/library_set/unmount_returns_adapter_and_drops_resolution.md) |
| related | [library_ids_yields_each_mount](/crates/oxide-library/src/adapters/library_set/library_ids_yields_each_mount.md) |
| related | [mount_rejects_duplicate_library_id_for_pathless_adapters](/crates/oxide-library/src/adapters/library_set/mount_rejects_duplicate_library_id_for_pathless_adapters.md) |
| related | [mount_allows_duplicate_library_id_at_different_paths](/crates/oxide-library/src/adapters/library_set/mount_allows_duplicate_library_id_at_different_paths.md) |
| related | [mount_rejects_duplicate_path](/crates/oxide-library/src/adapters/library_set/mount_rejects_duplicate_path.md) |
| related | [unmount_by_path_removes_specific_mount](/crates/oxide-library/src/adapters/library_set/unmount_by_path_removes_specific_mount.md) |
| related | [library_paths_skips_pathless_mounts](/crates/oxide-library/src/adapters/library_set/library_paths_skips_pathless_mounts.md) |
| related | [remount_replaces_previous_adapter_and_returns_old](/crates/oxide-library/src/adapters/library_set/remount_replaces_previous_adapter_and_returns_old.md) |
