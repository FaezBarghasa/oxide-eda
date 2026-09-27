---
okf_version: "0.2"
type: Function
title: apply_cascade_bump
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/apply_cascade_bump
language: rust
---

# apply_cascade_bump

## Signature

```rust
fn apply_cascade_bump(row: &mut ComponentRow, new_version: &str, kind: PrimitiveKindTag)
```

## Source
Lines 208–216 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| calls | [patch_bump](/crates/oxide-library/src/cascade/patch_bump.md) |
| called_by | [cascade_after_save](/crates/oxide-library/src/cascade/cascade_after_save.md) |
