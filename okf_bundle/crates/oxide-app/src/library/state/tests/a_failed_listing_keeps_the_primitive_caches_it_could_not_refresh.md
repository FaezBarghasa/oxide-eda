---
okf_version: "0.2"
type: Function
title: a_failed_listing_keeps_the_primitive_caches_it_could_not_refresh
description: "`reload_primitives` used to assign `Vec::new()` on every `Err` arm,"
resource: crates/oxide-app/src/library/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/state/tests/a_failed_listing_keeps_the_primitive_caches_it_could_not_refresh
language: rust
---

# a_failed_listing_keeps_the_primitive_caches_it_could_not_refresh

`reload_primitives` used to assign `Vec::new()` on every `Err` arm,

## Signature

```rust
fn a_failed_listing_keeps_the_primitive_caches_it_could_not_refresh()
```

## Decorators

- `test`

## Docstring

`reload_primitives` used to assign `Vec::new()` on every `Err` arm,
so one failed listing reported a mounted library as having no
symbols at all and the user re-created a primitive already on disk.
[test]

## Source
Lines 246–261 in `crates/oxide-app/src/library/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/state/tests.md) |
| calls | [library_with_cached_primitives](/crates/oxide-app/src/library/state/tests/library_with_cached_primitives.md) |
