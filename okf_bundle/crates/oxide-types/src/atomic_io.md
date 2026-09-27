---
okf_version: "0.2"
type: Module
title: atomic_io
description: Atomic file write helper used across the workspace (HI-6).
resource: crates/oxide-types/src/atomic_io.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/atomic_io
language: rust
---

# atomic_io

Atomic file write helper used across the workspace (HI-6).

## Docstring

Atomic file write helper used across the workspace (HI-6).

Crash-safety contract: a power loss or process crash mid-write
leaves the file system in one of two states only — the original
file untouched, or the new bytes fully present at the destination.
There is no half-written-file outcome.

Implementation: write to a `<path>.<pid>-<counter>.tmp` sibling,
`fsync` it, then `rename(.tmp, path)`, then `fsync` the parent
directory. The pid + per-process counter suffix keeps concurrent
writers to the same target from colliding on one temp name (#416);
the temp file still lives next to `path`, so the rename stays a
same-filesystem atomic-replace.
`std::fs::rename` is atomic-replace on every platform we ship —
POSIX `rename(2)` by spec, Windows `MoveFileExW` with
`MOVEFILE_REPLACE_EXISTING`. A crash between the write and the
rename leaves the original at the destination AND a stranded `.tmp`
sibling. The name is unique per writer now (#416), so a *crashed*
writer's temp is not auto-reclaimed by a later save — harmless
clutter, not lost data; every in-process failure path cleans its own
temp up.

The two `fsync`s are what make the power-loss guarantee real:
without fsyncing the temp file before the rename, many filesystems
(ext4 `data=writeback`, xfs, apfs) can persist the rename's
metadata before the file's data, so a power loss leaves a
zero-length or partially-written file at the destination. Fsyncing
the temp file forces the bytes down first; fsyncing the parent
directory forces the rename itself to be durable.

NB: `remove_file → rename` two-step would open a window where a
crash between the two calls leaves NO file at the destination and
a stranded `.tmp`, breaking the invariant. Don't reintroduce that.

## Relationships

| Type | Target |
|------|--------|
| related | [tmp_path_for](/crates/oxide-types/src/atomic_io/tmp_path_for.md) |
| related | [atomic_write](/crates/oxide-types/src/atomic_io/atomic_write.md) |
| related | [has_stray_tmp](/crates/oxide-types/src/atomic_io/has_stray_tmp.md) |
| related | [round_trip_creates_destination](/crates/oxide-types/src/atomic_io/round_trip_creates_destination.md) |
| related | [overwrites_existing_file](/crates/oxide-types/src/atomic_io/overwrites_existing_file.md) |
| related | [creates_parent_directory](/crates/oxide-types/src/atomic_io/creates_parent_directory.md) |
| related | [writes_empty_and_large_payloads_and_leaves_no_tmp](/crates/oxide-types/src/atomic_io/writes_empty_and_large_payloads_and_leaves_no_tmp.md) |
| related | [tmp_path_for_is_unique_per_call](/crates/oxide-types/src/atomic_io/tmp_path_for_is_unique_per_call.md) |
| related | [a_post_creation_failure_leaves_no_stray_tmp](/crates/oxide-types/src/atomic_io/a_post_creation_failure_leaves_no_stray_tmp.md) |
