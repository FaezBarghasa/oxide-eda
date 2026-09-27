---
okf_version: "0.2"
type: Function
title: cascade_after_symbol_save
description: "Run the cascade after a `save_symbol(sym, _)` succeeded."
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/cascade_after_symbol_save
language: rust
---

# cascade_after_symbol_save

Run the cascade after a `save_symbol(sym, _)` succeeded.

## Signature

```rust
pub fn cascade_after_symbol_save(
    adapter: &dyn LibraryAdapter,
    sym_uuid: Uuid,
    new_version: &str,
    mode: WorkflowMode,
) -> Result<CascadeReport, LibraryError>
```

## Visibility

- `pub`

## Docstring

Run the cascade after a `save_symbol(sym, _)` succeeded.

Walks every row across every table; rows whose `symbol_ref.uuid`
matches `sym_uuid` are bucketed and updated per [`WorkflowMode`].
Auto-bumped rows are written back via
[`LibraryAdapter::update_row`] with a synthesised commit message;
the message text is documented on
[`cascade_commit_message`].

## Source
Lines 97–110 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| calls | [cascade_after_save](/crates/oxide-library/src/cascade/cascade_after_save.md) |
| called_by | [save_symbol](/crates/oxide-library/src/adapters/local_git/adapter/save_symbol.md) |
| called_by | [cascade_team_mode_leaves_released_row_stale](/crates/oxide-library/tests/local_git_adapter/cascade_team_mode_leaves_released_row_stale.md) |
