---
okf_version: "0.2"
type: Function
title: diff_pin_map
resource: crates/oxide-library/src/diff.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/diff/diff_pin_map
language: rust
---

# diff_pin_map

## Signature

```rust
fn diff_pin_map(a: &[PinPadOverride], b: &[PinPadOverride]) -> PinMapDiff
```

## Source
Lines 165–207 in `crates/oxide-library/src/diff.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff](/crates/oxide-library/src/diff.md) |
| called_by | [diff_rows](/crates/oxide-library/src/diff/diff_rows.md) |
