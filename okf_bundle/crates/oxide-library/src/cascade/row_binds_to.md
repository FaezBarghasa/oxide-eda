---
okf_version: "0.2"
type: Function
title: row_binds_to
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/row_binds_to
language: rust
---

# row_binds_to

## Signature

```rust
fn row_binds_to(row: &ComponentRow, primitive_uuid: Uuid, kind: PrimitiveKindTag) -> bool
```

## Source
Lines 194–206 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| called_by | [cascade_after_save](/crates/oxide-library/src/cascade/cascade_after_save.md) |
