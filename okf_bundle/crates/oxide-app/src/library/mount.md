---
okf_version: "0.2"
type: Module
title: mount
description: "Off-thread `.snxlib` mount — issue #99 part 2c."
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount
language: rust
---

# mount

Off-thread `.snxlib` mount — issue #99 part 2c.

## Docstring

Off-thread `.snxlib` mount — issue #99 part 2c.

`update()` must never block on disk IO (MVU rule 3). Mounting a
`.snxlib` was the last synchronous file open of any size left in the
dispatcher: `LocalGitAdapter::open` parses the library file, then
`OpenLibrary::reload_tables` reads every TSV and walks `symbols/` /
`footprints/` / `sims/`. A six-medium-library project open still cost
**826.229 ms** after #528 dropped the duplicate scan — roughly 50
dropped frames at 60 Hz, and a single mount already crosses one frame
at ~58 symbols + 58 footprints. The work is CPU-parse-bound, so
several libraries prepare in parallel.

Shape mirrors the two conversions already in the repo
(`open_schematic_file` / `open_pcb_file` in
`app/handlers/document_files/open.rs`): `Task::perform` +
`tokio::task::spawn_blocking`, an in-flight marker taken before the
spawn, and a completion message carrying what the handler needs.
Two things differ, both deliberate:

1. The in-flight marker is a **map to an intent**, not a path set.
The same `.snxlib` can be requested by a project auto-mount (no
tab) and by the user double-clicking it (tab follows), and those
two requests race. See [`MountIntent`].
2. The payload is not `Clone`. `Message: Clone` but
`Box<dyn LibraryAdapter>` is not, so it travels in a
[`PreparedMountCell`] the handler `.take()`s. Ugly, and the
alternative is worse: re-running `LocalGitAdapter::open` on the UI
thread at completion time reintroduces exactly the block this
module exists to remove.

## Relationships

| Type | Target |
|------|--------|
| related | [MountIntent](/crates/oxide-app/src/library/mount/MountIntent.md) |
| related | [upgrade](/crates/oxide-app/src/library/mount/upgrade.md) |
| related | [upgrade](/crates/oxide-app/src/library/mount/upgrade.md) |
| related | [PreparedMount](/crates/oxide-app/src/library/mount/PreparedMount.md) |
| related | [path](/crates/oxide-app/src/library/mount/path.md) |
| related | [path](/crates/oxide-app/src/library/mount/path.md) |
| related | [PreparedMountCell](/crates/oxide-app/src/library/mount/PreparedMountCell.md) |
| related | [new](/crates/oxide-app/src/library/mount/new.md) |
| related | [take](/crates/oxide-app/src/library/mount/take.md) |
| related | [new](/crates/oxide-app/src/library/mount/new.md) |
| related | [take](/crates/oxide-app/src/library/mount/take.md) |
| related | [fmt](/crates/oxide-app/src/library/mount/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/mount/fmt.md) |
| related | [prepare_mount](/crates/oxide-app/src/library/mount/prepare_mount.md) |
| related | [adapter_ref](/crates/oxide-app/src/library/mount/adapter_ref.md) |
| related | [MountRequest](/crates/oxide-app/src/library/mount/MountRequest.md) |
| related | [request_mount](/crates/oxide-app/src/library/mount/request_mount.md) |
| related | [take_mount_intent](/crates/oxide-app/src/library/mount/take_mount_intent.md) |
| related | [mount_prepared](/crates/oxide-app/src/library/mount/mount_prepared.md) |
| related | [request_mount](/crates/oxide-app/src/library/mount/request_mount.md) |
| related | [take_mount_intent](/crates/oxide-app/src/library/mount/take_mount_intent.md) |
| related | [mount_prepared](/crates/oxide-app/src/library/mount/mount_prepared.md) |
| related | [prepare_mount_off_thread](/crates/oxide-app/src/library/mount/prepare_mount_off_thread.md) |
