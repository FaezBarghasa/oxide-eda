---
okf_version: "0.2"
type: Function
title: engine
resource: crates/oxide-engine/src/exec/batch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/batch/engine
language: rust
---

# engine

## Signature

```rust
fn engine() -> Engine
```

## Source
Lines 134–136 in `crates/oxide-engine/src/exec/batch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [batch](/crates/oxide-engine/src/exec/batch.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| called_by | [a_batch_at_the_history_cap_costs_exactly_one_undo_slot](/crates/oxide-engine/src/exec/batch/a_batch_at_the_history_cap_costs_exactly_one_undo_slot.md) |
| called_by | [a_batch_leaves_earlier_history_reachable](/crates/oxide-engine/src/exec/batch/a_batch_leaves_earlier_history_reachable.md) |
| called_by | [a_batch_longer_than_the_history_cap_is_still_one_entry](/crates/oxide-engine/src/exec/batch/a_batch_longer_than_the_history_cap_is_still_one_entry.md) |
| called_by | [a_batch_of_three_places_is_one_undo_step](/crates/oxide-engine/src/exec/batch/a_batch_of_three_places_is_one_undo_step.md) |
| called_by | [a_batch_that_changes_nothing_records_no_entry_and_keeps_redo](/crates/oxide-engine/src/exec/batch/a_batch_that_changes_nothing_records_no_entry_and_keeps_redo.md) |
| called_by | [a_changed_batch_clears_the_redo_stack](/crates/oxide-engine/src/exec/batch/a_changed_batch_clears_the_redo_stack.md) |
| called_by | [an_empty_batch_is_unchanged](/crates/oxide-engine/src/exec/batch/an_empty_batch_is_unchanged.md) |
| called_by | [redo_restores_the_whole_batch](/crates/oxide-engine/src/exec/batch/redo_restores_the_whole_batch.md) |
| called_by | [semantic_is_kept_when_every_command_agrees](/crates/oxide-engine/src/exec/batch/semantic_is_kept_when_every_command_agrees.md) |
| called_by | [semantic_is_widened_to_document_replaced_when_the_batch_mixes_kinds](/crates/oxide-engine/src/exec/batch/semantic_is_widened_to_document_replaced_when_the_batch_mixes_kinds.md) |
| called_by | [the_returned_document_patch_is_the_union_of_the_batch](/crates/oxide-engine/src/exec/batch/the_returned_document_patch_is_the_union_of_the_batch.md) |
| called_by | [the_suppression_flag_is_cleared_so_later_commands_still_record](/crates/oxide-engine/src/exec/batch/the_suppression_flag_is_cleared_so_later_commands_still_record.md) |
| called_by | [unchanged_commands_inside_a_batch_do_not_dilute_the_patch](/crates/oxide-engine/src/exec/batch/unchanged_commands_inside_a_batch_do_not_dilute_the_patch.md) |
