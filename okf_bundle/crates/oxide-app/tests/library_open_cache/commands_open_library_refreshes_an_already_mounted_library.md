---
okf_version: "0.2"
type: Function
title: commands_open_library_refreshes_an_already_mounted_library
description: "Order-independent fingerprint of the five cached fields — row ids,"
resource: crates/oxide-app/tests/library_open_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:14Z"
concept_id: crates/oxide-app/tests/library_open_cache/commands_open_library_refreshes_an_already_mounted_library
language: rust
---

# commands_open_library_refreshes_an_already_mounted_library

Order-independent fingerprint of the five cached fields — row ids,

## Signature

```rust
fn commands_open_library_refreshes_an_already_mounted_library()
```

## Decorators

- `test`

## Docstring

Order-independent fingerprint of the five cached fields — row ids,
#530 — the same warm/cold split, on the *interactive* path.

`commands::open_library` now refreshes only when the library was
already mounted. The cold half is covered by
`refresh_components_after_open_changes_nothing` above (the refresh was
a no-op there, which is why dropping it is safe); this pins the warm
half, which is the part a blanket deletion would break.

Every caller can reach this branch: `self.library` is app-global, so
re-opening an already-mounted library from the Components Panel, the
primitive picker, the lifecycle handler or a project save all land
here.
[test]

## Source
Lines 286–324 in `crates/oxide-app/tests/library_open_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_open_cache](/crates/oxide-app/tests/library_open_cache.md) |
| calls | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
| calls | [append_component](/crates/oxide-app/tests/support/mod/append_component.md) |
