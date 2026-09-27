---
okf_version: "0.2"
type: Module
title: undo_marker_divergence
description: "#533 — the engine owns the undo history."
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence
language: rust
---

# undo_marker_divergence

#533 — the engine owns the undo history.

## Docstring

#533 — the engine owns the undo history.

There used to be two histories that had to advance in lockstep and
did not:

* the **engine's** own, pushed inside `Engine::execute` via
`record_history` — it happens whatever the caller does with the
returned `CommandResult`;
* the app's **marker stack** (`UndoStack` in the since-deleted
`crates/oxide-app/src/undo.rs`), pushed only by
`mutation_gateway.rs` from `CommandResult::changed`.

`apply_engine_undo` was driven by the marker stack: it peeked how
many engine steps the top marker covered, then called `Engine::undo`
that many times. The Edit menu's Undo item, however, is enabled from
the *engine* (`app/view/mod.rs` reads `e.can_undo()`).

So a call site that ran `engine.execute(...)` directly and dropped
the `CommandResult` pushed an engine entry with no marker to match.
The stacks slipped by one, the oldest edit could no longer be
reached, and Undo went on claiming to be available while doing
nothing. The stack was also one *global* stack across every open
document's per-path engine, never cleared on tab switch, so its
counts could not be right across tabs even in principle.

The marker stack is gone. `apply_engine_undo` / `apply_engine_redo`
call `Engine::undo` / `Engine::redo` once, and batches are one engine
history entry because `apply_engine_commands` goes through
`Engine::execute_batch`. These tests pin down the four properties
that used to break.

`handle_move_selection_apply` (`app/handlers/erc/modals.rs:304`) is
one of the five sites #533 lists under Class B, and is used below
precisely because it still runs the engine directly — it is correct
now without having been touched. The others are `modals.rs:325` and
`erc/annotate.rs:77,268,296`.

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/tests/regression/undo_marker_divergence/symbol.md) |
| related | [sheet_with](/crates/oxide-app/tests/regression/undo_marker_divergence/sheet_with.md) |
| related | [add_tab](/crates/oxide-app/tests/regression/undo_marker_divergence/add_tab.md) |
| related | [activate](/crates/oxide-app/tests/regression/undo_marker_divergence/activate.md) |
| related | [symbols_of](/crates/oxide-app/tests/regression/undo_marker_divergence/symbols_of.md) |
| related | [fixture_two_symbols](/crates/oxide-app/tests/regression/undo_marker_divergence/fixture_two_symbols.md) |
| related | [one_gateway_edit_then_one_bypassing_edit](/crates/oxide-app/tests/regression/undo_marker_divergence/one_gateway_edit_then_one_bypassing_edit.md) |
| related | [two_edits_then_two_undos_restores_both](/crates/oxide-app/tests/regression/undo_marker_divergence/two_edits_then_two_undos_restores_both.md) |
| related | [undo_availability_agrees_with_what_undo_can_actually_do](/crates/oxide-app/tests/regression/undo_marker_divergence/undo_availability_agrees_with_what_undo_can_actually_do.md) |
| related | [a_pasted_batch_is_one_undo_step](/crates/oxide-app/tests/regression/undo_marker_divergence/a_pasted_batch_is_one_undo_step.md) |
| related | [an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits](/crates/oxide-app/tests/regression/undo_marker_divergence/an_undo_in_one_tab_spends_exactly_one_of_that_tabs_edits.md) |
