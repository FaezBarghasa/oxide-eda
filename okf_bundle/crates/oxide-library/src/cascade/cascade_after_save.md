---
okf_version: "0.2"
type: Function
title: cascade_after_save
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/cascade_after_save
language: rust
---

# cascade_after_save

## Signature

```rust
fn cascade_after_save(
    adapter: &dyn LibraryAdapter,
    primitive_uuid: Uuid,
    new_version: &str,
    mode: WorkflowMode,
    kind: PrimitiveKindTag,
) -> Result<CascadeReport, LibraryError>
```

## Source
Lines 165–192 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| calls | [row_binds_to](/crates/oxide-library/src/cascade/row_binds_to.md) |
| calls | [apply_cascade_bump](/crates/oxide-library/src/cascade/apply_cascade_bump.md) |
| calls | [cascade_commit_message](/crates/oxide-library/src/cascade/cascade_commit_message.md) |
| called_by | [cascade_after_footprint_save](/crates/oxide-library/src/cascade/cascade_after_footprint_save.md) |
| called_by | [cascade_after_sim_save](/crates/oxide-library/src/cascade/cascade_after_sim_save.md) |
| called_by | [cascade_after_symbol_save](/crates/oxide-library/src/cascade/cascade_after_symbol_save.md) |
