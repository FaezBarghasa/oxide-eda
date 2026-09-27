---
okf_version: "0.2"
type: Module
title: param_commit
description: "What a Library Browser parameter cell commits, given what it holds."
resource: crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/param_commit
language: rust
---

# param_commit

What a Library Browser parameter cell commits, given what it holds.

## Docstring

What a Library Browser parameter cell commits, given what it holds.

A cell that already holds a typed value keeps that type. Before #599
the `Err` arm of the numeric parse substituted
`ParamValue::Text(buffer)`, which silently retyped the parameter and
threw the stored unit away: `Measurement { 4.7, "kΩ" }` edited to
`4k7` became `Text("4k7")`, and the cell still rendered the typed
text so it looked committed. What was gone was the numeric type and
unit that every numeric facet filter, range query, sort and BOM
column keys on — the row dropped out of "resistance between 1k and
10k" for good.

An unparseable buffer is now a refusal: the stored value is left
alone and the caller reports it and keeps the buffer in the cell so
the user can correct it.

#612 extended that to the boolean arm, which the register had not
recorded and which was retyping the same way — a buffer that is
neither `true` nor `false` used to become `Text` and take the
boolean type with it.

## Relationships

| Type | Target |
|------|--------|
| related | [ParamRefusal](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/ParamRefusal.md) |
| related | [param_value_for_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/param_value_for_commit.md) |
| related | [refusal](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/refusal.md) |
| related | [kilo_ohms](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/kilo_ohms.md) |
| related | [a_measurement_cell_keeps_its_unit_when_the_buffer_parses](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_measurement_cell_keeps_its_unit_when_the_buffer_parses.md) |
| related | [a_measurement_cell_refuses_a_buffer_that_is_not_a_number](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_measurement_cell_refuses_a_buffer_that_is_not_a_number.md) |
| related | [a_number_cell_refuses_a_buffer_that_is_not_a_number](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_number_cell_refuses_a_buffer_that_is_not_a_number.md) |
| related | [a_number_cell_commits_a_parseable_buffer](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_number_cell_commits_a_parseable_buffer.md) |
| related | [an_untyped_cell_takes_the_buffer_verbatim](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/an_untyped_cell_takes_the_buffer_verbatim.md) |
| related | [a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping.md) |
| related | [a_bool_cell_still_takes_true_and_false_in_any_case](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_bool_cell_still_takes_true_and_false_in_any_case.md) |
| related | [refusing_a_boolean_buffer_does_not_touch_untyped_cells](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/refusing_a_boolean_buffer_does_not_touch_untyped_cells.md) |
