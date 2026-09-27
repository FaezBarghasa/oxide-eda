---
okf_version: "0.2"
type: Function
title: assert_dirty
resource: crates/oxide-app/tests/regression/dirty_paths_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/dirty_paths_gateway/assert_dirty
language: rust
---

# assert_dirty

## Signature

```rust
fn assert_dirty(app: &Oxide, path: &Path, what: &str)
```

## Source
Lines 109–121 in `crates/oxide-app/tests/regression/dirty_paths_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dirty_paths_gateway](/crates/oxide-app/tests/regression/dirty_paths_gateway.md) |
| called_by | [a_move_selection_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_move_selection_marks_the_document_dirty.md) |
| called_by | [a_parameter_manager_edit_marks_the_document_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/a_parameter_manager_edit_marks_the_document_dirty.md) |
| called_by | [annotate_marks_the_active_sheet_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/annotate_marks_the_active_sheet_dirty.md) |
| called_by | [reset_duplicate_designators_marks_the_active_sheet_dirty](/crates/oxide-app/tests/regression/dirty_paths_gateway/reset_duplicate_designators_marks_the_active_sheet_dirty.md) |
