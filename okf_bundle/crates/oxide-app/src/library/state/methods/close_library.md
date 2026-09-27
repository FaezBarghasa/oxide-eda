---
okf_version: "0.2"
type: Function
title: close_library
description: "Drop the library backing `root` — unmounts from `set` and drops"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/close_library
language: rust
---

# close_library

Drop the library backing `root` — unmounts from `set` and drops

## Signature

```rust
impl LibraryState { pub fn close_library(&mut self, root: &Path) }
```

## Visibility

- `pub`

## Docstring

Drop the library backing `root` — unmounts from `set` and drops
every editor pointing at it. (TODO(v0.9): unsaved-edits prompt
is wired from the dispatcher via `dirty_editors_for_library`.)

## Source
Lines 168–186 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
