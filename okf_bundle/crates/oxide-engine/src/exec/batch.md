---
okf_version: "0.2"
type: Module
title: batch
description: "`Engine::execute_batch` — run N commands as one undoable step."
resource: crates/oxide-engine/src/exec/batch.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/batch
language: rust
---

# batch

`Engine::execute_batch` — run N commands as one undoable step.

## Docstring

`Engine::execute_batch` — run N commands as one undoable step.

Pasting 40 objects, or a find/replace that rewrites 12 labels, has to
undo as one action rather than forty. Until now that grouping lived in
an app-side marker stack that recorded how many engine steps each user
action was worth. That stack was global across the per-path engines and
was never cleared on tab switch, so its counts drifted out of step with
the history they were counting and the app could revert a document
without reporting it (#533). Grouping belongs next to the history it
groups, which is here.

## Why the inner writes are suppressed rather than truncated afterwards

The obvious shape is to let each inner `execute` record its own entry
and then truncate the batch's entries away and push one coalesced entry
in their place. That silently erodes undo depth. `record_history` evicts
from the *front* once `history` reaches `MAX_HISTORY_ENTRIES`, so a batch
of 10 run against a full history evicts 10 genuine pre-batch entries on
the way in. Truncating the 10 transient entries afterwards cannot bring
those back: the user is left at depth 91 after an action that should have
cost one slot, and a batch longer than the cap wipes the pre-batch
history outright. Suppressing the inner writes up front means the
transient entries never exist, so the batch causes exactly one eviction —
the correct number. It also drops the truncate shape's reliance on the
unenforced "exactly one `record_history` per changed result" convention,
which a future handler could break without any test noticing.

## Relationships

| Type | Target |
|------|--------|
| related | [execute_batch](/crates/oxide-engine/src/exec/batch/execute_batch.md) |
| related | [run_batch](/crates/oxide-engine/src/exec/batch/run_batch.md) |
| related | [execute_batch](/crates/oxide-engine/src/exec/batch/execute_batch.md) |
| related | [run_batch](/crates/oxide-engine/src/exec/batch/run_batch.md) |
| related | [engine](/crates/oxide-engine/src/exec/batch/engine.md) |
| related | [place_no_connect](/crates/oxide-engine/src/exec/batch/place_no_connect.md) |
| related | [place_label](/crates/oxide-engine/src/exec/batch/place_label.md) |
| related | [no_op](/crates/oxide-engine/src/exec/batch/no_op.md) |
| related | [a_batch_of_three_places_is_one_undo_step](/crates/oxide-engine/src/exec/batch/a_batch_of_three_places_is_one_undo_step.md) |
| related | [an_empty_batch_is_unchanged](/crates/oxide-engine/src/exec/batch/an_empty_batch_is_unchanged.md) |
| related | [redo_restores_the_whole_batch](/crates/oxide-engine/src/exec/batch/redo_restores_the_whole_batch.md) |
| related | [a_batch_leaves_earlier_history_reachable](/crates/oxide-engine/src/exec/batch/a_batch_leaves_earlier_history_reachable.md) |
| related | [a_batch_that_changes_nothing_records_no_entry_and_keeps_redo](/crates/oxide-engine/src/exec/batch/a_batch_that_changes_nothing_records_no_entry_and_keeps_redo.md) |
| related | [a_changed_batch_clears_the_redo_stack](/crates/oxide-engine/src/exec/batch/a_changed_batch_clears_the_redo_stack.md) |
| related | [the_returned_document_patch_is_the_union_of_the_batch](/crates/oxide-engine/src/exec/batch/the_returned_document_patch_is_the_union_of_the_batch.md) |
| related | [semantic_is_kept_when_every_command_agrees](/crates/oxide-engine/src/exec/batch/semantic_is_kept_when_every_command_agrees.md) |
| related | [semantic_is_widened_to_document_replaced_when_the_batch_mixes_kinds](/crates/oxide-engine/src/exec/batch/semantic_is_widened_to_document_replaced_when_the_batch_mixes_kinds.md) |
| related | [a_batch_at_the_history_cap_costs_exactly_one_undo_slot](/crates/oxide-engine/src/exec/batch/a_batch_at_the_history_cap_costs_exactly_one_undo_slot.md) |
| related | [a_batch_longer_than_the_history_cap_is_still_one_entry](/crates/oxide-engine/src/exec/batch/a_batch_longer_than_the_history_cap_is_still_one_entry.md) |
| related | [unchanged_commands_inside_a_batch_do_not_dilute_the_patch](/crates/oxide-engine/src/exec/batch/unchanged_commands_inside_a_batch_do_not_dilute_the_patch.md) |
| related | [the_suppression_flag_is_cleared_so_later_commands_still_record](/crates/oxide-engine/src/exec/batch/the_suppression_flag_is_cleared_so_later_commands_still_record.md) |
