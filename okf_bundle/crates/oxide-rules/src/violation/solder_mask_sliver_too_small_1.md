---
okf_version: "0.2"
type: Function
title: solder_mask_sliver_too_small
resource: crates/oxide-rules/src/violation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-rules"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T05:27:50Z"
concept_id: crates/oxide-rules/src/violation/solder_mask_sliver_too_small_1
language: rust
---

# solder_mask_sliver_too_small

## Signature

```rust
pub fn solder_mask_sliver_too_small(
        object_id: &str,
        scope: RuleScope,
        min_sliver_microns: i64,
        actual_sliver_microns: i64,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 84–103 in `crates/oxide-rules/src/violation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [violation](/crates/oxide-rules/src/violation.md) |
