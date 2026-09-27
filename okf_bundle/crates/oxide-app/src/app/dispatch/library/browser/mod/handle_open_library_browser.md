---
okf_version: "0.2"
type: Function
title: handle_open_library_browser
description: "Open `.snxlib` at `path` as a Library Browser tab."
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_open_library_browser
language: rust
---

# handle_open_library_browser

Open `.snxlib` at `path` as a Library Browser tab.

## Signature

```rust
impl Oxide { pub(crate) fn handle_open_library_browser(
        &mut self,
        path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Open `.snxlib` at `path` as a Library Browser tab.

Step 1 — the mount — now happens **off the UI thread** (#99 part
2c). `LocalGitAdapter::open` plus the table/primitive scans are
the expensive half of this gesture and blocking `update()` on them
dropped frames; the tab opens one update tick later instead, when
`LibraryMessage::MountFinished` lands and
[`Self::finish_open_library_browser`] runs steps 2 and 3.

Three outcomes, and conflating any two of them is a bug:
already-mounted finishes immediately, an in-flight preparation
does nothing (its completion will finish, on the upgraded intent),
and only a fresh request spawns.

## Source
Lines 33–76 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [prepare_mount_off_thread](/crates/oxide-app/src/library/mount/prepare_mount_off_thread.md) |
