---
okf_version: "0.2"
type: Function
title: cascade_footprint_personal_mode_auto_bumps_bound_row
description: Footprint cascade mirrors symbol cascade — same predicate
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/cascade_footprint_personal_mode_auto_bumps_bound_row
language: rust
---

# cascade_footprint_personal_mode_auto_bumps_bound_row

Footprint cascade mirrors symbol cascade — same predicate

## Signature

```rust
fn cascade_footprint_personal_mode_auto_bumps_bound_row()
```

## Decorators

- `test`

## Docstring

Footprint cascade mirrors symbol cascade — same predicate
(Personal mode OR `!row.released`), same auto-bump rules. Test
confirms the trait dispatches against `footprint_ref` rather
than `symbol_ref`.
[test]

## Source
Lines 1077–1117 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [snxlib_path](/crates/oxide-library/tests/local_git_adapter/snxlib_path.md) |
| calls | [snx_manifest_with_mode](/crates/oxide-library/tests/local_git_adapter/snx_manifest_with_mode.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
| calls | [fixture_row](/crates/oxide-library/tests/local_git_adapter/fixture_row.md) |
