---
okf_version: "0.2"
type: Function
title: refresh_components_keeps_the_caches_when_the_listings_fail
description: "`refresh_components` swallowed the same three calls with"
resource: crates/oxide-app/src/library/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/state/tests/refresh_components_keeps_the_caches_when_the_listings_fail
language: rust
---

# refresh_components_keeps_the_caches_when_the_listings_fail

`refresh_components` swallowed the same three calls with

## Signature

```rust
fn refresh_components_keeps_the_caches_when_the_listings_fail()
```

## Decorators

- `test`

## Docstring

`refresh_components` swallowed the same three calls with
`unwrap_or_default()` and no log at all — the empty vecs landed in
the caches the primitive picker renders.
[test]

## Source
Lines 267–291 in `crates/oxide-app/src/library/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/state/tests.md) |
| calls | [library_with_cached_primitives](/crates/oxide-app/src/library/state/tests/library_with_cached_primitives.md) |
