---
okf_version: "0.2"
type: Function
title: normalize_arc_endpoints_deg
description: "Normalise an `Arc`'s `start_deg`/`end_deg` into this codebase's"
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/normalize_arc_endpoints_deg
language: rust
---

# normalize_arc_endpoints_deg

Normalise an `Arc`'s `start_deg`/`end_deg` into this codebase's

## Signature

```rust
pub fn normalize_arc_endpoints_deg(start_deg: f64, end_deg: f64) -> (f64, f64)
```

## Visibility

- `pub`

## Docstring

Normalise an `Arc`'s `start_deg`/`end_deg` into this codebase's
counter-clockwise-wraparound sweep convention: `start..end` sweeps
CCW (increasing angle) from `start`, wrapping through a full turn
when `end < start` — the convention the symbol canvas's arc
hit-test, its Arc rotate transform, and the GPU arc shader all
already assume. Returns the pair with a swap applied when needed
(see below), then both endpoints reduced into a canonical
`[0, 360)` range.

Why a swap and not just reducing each field into range: reducing
`start_deg`/`end_deg` independently only ever changes each by a
whole multiple of 360°, so it can't change `end_deg - start_deg`
by anything but a multiple of 360° either — the CCW-wraparound
sweep would stay exactly what it was. When the pair actually
represents a clockwise-signed drag rather than an intentionally
wrapped arc, that unchanged sweep is the WRONG one — the
complement of the arc that was meant. Swapping instead re-orders
the pair so its wraparound sweep becomes the complement
(`360° - old_sweep`), which is the short arc that was intended.

Used by two call sites that can't share a dependency edge: this
crate's own [`SymbolFile::from_toml_str`] (migrating legacy
`.snxsym` files saved by builds that stored a clockwise drag's
raw, unswapped pair) and `oxide_app`'s Place Arc placement-commit
handler (the tool that can hand this a raw, possibly-negative pair
from a live drag). Lives here — oxide-library must not depend on
oxide-app — with oxide-app calling into it, not the reverse.

## Source
Lines 261–268 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
| called_by | [normalize_arc_commit_deg](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg.md) |
| called_by | [migrate_legacy_arc](/crates/oxide-library/src/primitive/symbol/mod/migrate_legacy_arc.md) |
