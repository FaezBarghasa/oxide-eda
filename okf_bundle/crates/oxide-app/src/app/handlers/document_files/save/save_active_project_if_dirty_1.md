---
okf_version: "0.2"
type: Function
title: save_active_project_if_dirty
description: "Persist the active project's `.snxprj` JSON when its dirty bit"
resource: crates/oxide-app/src/app/handlers/document_files/save.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/save/save_active_project_if_dirty_1
language: rust
---

# save_active_project_if_dirty

Persist the active project's `.snxprj` JSON when its dirty bit

## Signature

```rust
fn save_active_project_if_dirty(&mut self)
```

## Docstring

Persist the active project's `.snxprj` JSON when its dirty bit
is set in `dirty_paths`. No-op if no project is active or the
project file isn't dirty.

Pending libraries (registered via the New Library flow) are
materialised to disk **first** — `commands::materialize_pending_library`
runs `LocalGitAdapter::init` for each entry. Successful
materialisations push their `LibraryEntry` onto
`project.libraries` so the subsequent `.snxprj` write captures
them. Failures (e.g. target path now exists, permission glitch)
keep the entry pending so the user can fix the underlying
problem and retry on the next Save.

Closes `feedback_no_disk_writes_without_user_save.md`'s "wait
for explicit user save" invariant: modal confirm registered the
pending entry; this Ctrl+S is the explicit save that actually
commits to disk.

## Source
Lines 115–134 in `crates/oxide-app/src/app/handlers/document_files/save.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [save](/crates/oxide-app/src/app/handlers/document_files/save.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
