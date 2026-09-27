---
okf_version: "0.2"
type: Function
title: cascade_after_sim_save
description: "Run the cascade after a `save_sim(sm, _)` succeeded."
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/cascade_after_sim_save
language: rust
---

# cascade_after_sim_save

Run the cascade after a `save_sim(sm, _)` succeeded.

## Signature

```rust
pub fn cascade_after_sim_save(
    adapter: &dyn LibraryAdapter,
    sim_uuid: Uuid,
    new_version: &str,
    mode: WorkflowMode,
) -> Result<CascadeReport, LibraryError>
```

## Visibility

- `pub`

## Docstring

Run the cascade after a `save_sim(sm, _)` succeeded.

Mirrors [`cascade_after_symbol_save`] but matches against the row's
`sim_ref` (an `Option<PrimitiveRef>` — rows without a sim binding
are skipped silently).

## Source
Lines 137–144 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| calls | [cascade_after_save](/crates/oxide-library/src/cascade/cascade_after_save.md) |
| called_by | [save_sim](/crates/oxide-library/src/adapters/local_git/adapter/save_sim.md) |
