---
okf_version: "0.2"
type: Function
title: handle_library_updates_apply
description: "Apply the user's selected updates from the Library Updates"
resource: crates/oxide-app/src/app/dispatch/library/updates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/updates/handle_library_updates_apply
language: rust
---

# handle_library_updates_apply

Apply the user's selected updates from the Library Updates

## Signature

```rust
impl Oxide { pub(crate) fn handle_library_updates_apply(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Apply the user's selected updates from the Library Updates
modal to the schematic engine. Drops the modal state on
success; on apply, dirty-marks the schematic and clears its
"skipped" indicator (the user committed an update, the path is
no longer ambiguously skipped).

## Source
Lines 222–260 in `crates/oxide-app/src/app/dispatch/library/updates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/app/dispatch/library/updates.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
