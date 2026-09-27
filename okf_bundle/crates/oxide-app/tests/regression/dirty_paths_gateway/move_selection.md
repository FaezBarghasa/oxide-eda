---
okf_version: "0.2"
type: Function
title: move_selection
description: "Apply a non-zero Move Selection to `uuid` through the dialog."
resource: crates/oxide-app/tests/regression/dirty_paths_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/dirty_paths_gateway/move_selection
language: rust
---

# move_selection

Apply a non-zero Move Selection to `uuid` through the dialog.

## Signature

```rust
fn move_selection(app: &mut Oxide, uuid: uuid::Uuid)
```

## Docstring

Apply a non-zero Move Selection to `uuid` through the dialog.

## Source
Lines 124–130 in `crates/oxide-app/tests/regression/dirty_paths_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dirty_paths_gateway](/crates/oxide-app/tests/regression/dirty_paths_gateway.md) |
| called_by | [a_move_selection_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_move_selection_marks_the_document_dirty.md) |
| called_by | [quitting_after_a_move_selection_warns_instead_of_discarding_it](/crates/oxide-app/tests/regression/dirty_paths_gateway/quitting_after_a_move_selection_warns_instead_of_discarding_it.md) |
