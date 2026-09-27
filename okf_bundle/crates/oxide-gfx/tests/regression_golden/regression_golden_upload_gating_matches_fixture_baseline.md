---
okf_version: "0.2"
type: Function
title: regression_golden_upload_gating_matches_fixture_baseline
description: "[test]"
resource: crates/oxide-gfx/tests/regression_golden.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-gfx/tests/regression_golden/regression_golden_upload_gating_matches_fixture_baseline
language: rust
---

# regression_golden_upload_gating_matches_fixture_baseline

[test]

## Signature

```rust
fn regression_golden_upload_gating_matches_fixture_baseline()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 430–542 in `crates/oxide-gfx/tests/regression_golden.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [regression_golden](/crates/oxide-gfx/tests/regression_golden.md) |
| calls | [load_golden_fixture](/crates/oxide-gfx/tests/regression_golden/load_golden_fixture.md) |
| calls | [build_upload_fixture_scene](/crates/oxide-gfx/tests/regression_golden/build_upload_fixture_scene.md) |
| calls | [apply_dirty_uploads](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads.md) |
| calls | [apply_dirty_uploads_with_culling](/crates/oxide-gfx/src/scene/upload/mod/apply_dirty_uploads_with_culling.md) |
