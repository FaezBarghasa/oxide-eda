---
okf_version: "0.2"
type: Function
title: row_with
resource: crates/oxide-library/tests/diff_golden.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/diff_golden/row_with
language: rust
---

# row_with

## Signature

```rust
fn row_with(symbol: PrimitiveRef, footprint: Option<PrimitiveRef>, mpn: &str) -> ComponentRow
```

## Source
Lines 23–51 in `crates/oxide-library/tests/diff_golden.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diff_golden](/crates/oxide-library/tests/diff_golden.md) |
| calls | [fixed_uuid](/crates/oxide-library/tests/diff_golden/fixed_uuid.md) |
| called_by | [diff_is_symmetric_on_added_supply](/crates/oxide-library/tests/diff_golden/diff_is_symmetric_on_added_supply.md) |
| called_by | [lifecycle_diff_records_state_change_when_present](/crates/oxide-library/tests/diff_golden/lifecycle_diff_records_state_change_when_present.md) |
| called_by | [mpn_only_swap_is_minor_bump](/crates/oxide-library/tests/diff_golden/mpn_only_swap_is_minor_bump.md) |
| called_by | [symbol_ref_swap_is_major_bump](/crates/oxide-library/tests/diff_golden/symbol_ref_swap_is_major_bump.md) |
