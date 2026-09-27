---
okf_version: "0.2"
type: Function
title: open_library
description: "Open a `*.snxlib/` and, when it was already mounted, refresh its"
resource: crates/oxide-app/src/library/commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/commands/open_library
language: rust
---

# open_library

Open a `*.snxlib/` and, when it was already mounted, refresh its

## Signature

```rust
pub fn open_library(state: &mut LibraryState, root: PathBuf) -> Result<(), LibraryError>
```

## Visibility

- `pub`

## Docstring

Open a `*.snxlib/` and, when it was already mounted, refresh its
component list.

The refresh is warm-path only — the same split #528 made in
`auto_mount_project_libraries`, applied here to the interactive path
(#530). Not a micro-optimisation: the duplicate scan measured
**128.653 ms** for a medium library (500 symbols + 500 footprints) and
**540.508 ms** for a large one (2000 + 2000), and every manual "open a
library" gesture paid it.

COLD (not yet mounted): [`LibraryState::open_library`] has just run
`reload_tables` → `reload_primitives`, priming all five caches off the
exact adapter calls a refresh would repeat. Refreshing here recomputes
identical values.

WARM (already mounted): `open_library` early-returns at
`state/methods.rs:83-85` and never reaches `reload_tables`, so this
refresh is the *only* thing that rescans. Dropping it unconditionally
would leave an already-mounted library showing a stale snapshot —
which is why this is a guard and not a deletion. Reachable from every
caller: `self.library` is app-global, so re-opening an
already-mounted library lands here.

## Source
Lines 50–57 in `crates/oxide-app/src/library/commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [commands](/crates/oxide-app/src/library/commands.md) |
| called_by | [create_library_at](/crates/oxide-app/src/library/commands/create_library_at.md) |
| called_by | [materialize_pending_library](/crates/oxide-app/src/library/commands/materialize_pending_library.md) |
