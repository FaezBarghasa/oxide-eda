---
okf_version: "0.2"
type: Function
title: apply_primitive_listing
description: "Move one primitive listing onto its cache, keeping the previous"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/apply_primitive_listing
language: rust
---

# apply_primitive_listing

Move one primitive listing onto its cache, keeping the previous

## Signature

```rust
fn apply_primitive_listing(
    cache: &mut Vec<PrimitiveSummary>,
    listing: Result<Vec<PrimitiveSummary>, LibraryError>,
    library: &str,
    kind: &str,
)
```

## Docstring

Move one primitive listing onto its cache, keeping the previous
contents when the adapter failed.

A listing failure is transient — a lock held by another process, a
library server 500, an unreadable `symbols/` directory — but the
cache it feeds is the *only* thing the picker renders. Substituting
an empty vec turns "the listing failed" into "this library has no
symbols", so the user re-creates a primitive that is already on
disk. The stale cache is the better answer: it is what the picker
showed a moment ago, and the next successful refresh replaces it.

The failure itself reaches the Messages panel at `warn` — the same
treatment `refresh_components` gives a failed `read_table`.

## Source
Lines 18–34 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
| called_by | [refresh_components](/crates/oxide-app/src/library/state/methods/refresh_components.md) |
| called_by | [reload_primitives](/crates/oxide-app/src/library/state/methods/reload_primitives.md) |
