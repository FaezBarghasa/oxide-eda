---
okf_version: "0.2"
type: Function
title: nudge_pads_skips_out_of_range_indices
description: Out-of-range indices are skipped (no panic) and excluded from the
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/nudge_pads_skips_out_of_range_indices
language: rust
---

# nudge_pads_skips_out_of_range_indices

Out-of-range indices are skipped (no panic) and excluded from the

## Signature

```rust
fn nudge_pads_skips_out_of_range_indices()
```

## Decorators

- `test`

## Docstring

Out-of-range indices are skipped (no panic) and excluded from the
returned moved-list — the dispatcher relies on this to mirror only
the pads that actually moved into the sketch.
[test]

## Source
Lines 113–119 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
