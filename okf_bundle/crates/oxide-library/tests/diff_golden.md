---
okf_version: "0.2"
type: Module
title: diff_golden
description: Integration tests for the v0.9-refactor-2 row-diff engine.
resource: crates/oxide-library/tests/diff_golden.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/diff_golden
language: rust
---

# diff_golden

Integration tests for the v0.9-refactor-2 row-diff engine.

## Docstring

Integration tests for the v0.9-refactor-2 row-diff engine.

Per `v0.9-refactor-2-plan.md` §6 step 1.7, the diff is pure-ref +
binding-field comparison over [`ComponentRow`] pairs. Geometry-level
diffs live with the primitive editors and are out of scope here.

These tests build three synthetic rows of the same internal_pn and
verify:
- mpn-only swap is a Minor bump,
- symbol_ref swap is a Major bump,
- the diff is symmetric (added/removed swap on reversal),
- lifecycle transitions surface in `lifecycle_detail`.

## Relationships

| Type | Target |
|------|--------|
| related | [fixed_uuid](/crates/oxide-library/tests/diff_golden/fixed_uuid.md) |
| related | [row_with](/crates/oxide-library/tests/diff_golden/row_with.md) |
| related | [mpn_only_swap_is_minor_bump](/crates/oxide-library/tests/diff_golden/mpn_only_swap_is_minor_bump.md) |
| related | [symbol_ref_swap_is_major_bump](/crates/oxide-library/tests/diff_golden/symbol_ref_swap_is_major_bump.md) |
| related | [diff_is_symmetric_on_added_supply](/crates/oxide-library/tests/diff_golden/diff_is_symmetric_on_added_supply.md) |
| related | [lifecycle_diff_records_state_change_when_present](/crates/oxide-library/tests/diff_golden/lifecycle_diff_records_state_change_when_present.md) |
