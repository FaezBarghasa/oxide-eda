---
okf_version: "0.2"
type: Function
title: validate_split
description: "Validate `line` / `t` against read-only lookups only — resolves the"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/validate_split
language: rust
---

# validate_split

Validate `line` / `t` against read-only lookups only — resolves the

## Signature

```rust
fn validate_split(
    sketch: &SketchData,
    line: SketchEntityId,
    t: f64,
) -> Result<ValidatedSplit, SplitError>
```

## Docstring

Validate `line` / `t` against read-only lookups only — resolves the
line's index and endpoints, checks `t`'s range, and rejects a line
no `t` could ever split (shorter than `2 * MIN_SEGMENT_LEN_MM` —
see [`SplitError::DegenerateLine`]'s doc for why `2x` and not `1x`).
The caller still owes the post-mid-point
[`SplitError::TooCloseToEndpoint`] / non-finite check, which needs
`t` applied to real coordinates rather than a lookup.

## Source
Lines 293–322 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
| calls | [entity_point_xy](/crates/oxide-sketch/src/split/mod/entity_point_xy.md) |
| calls | [mm_distance](/crates/oxide-sketch/src/split/mod/mm_distance.md) |
| called_by | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
