---
okf_version: "0.2"
type: Function
title: no_op
description: "`DeleteSelection` with nothing selected takes the `if !changed` early"
resource: crates/oxide-engine/src/exec/batch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/batch/no_op
language: rust
---

# no_op

`DeleteSelection` with nothing selected takes the `if !changed` early

## Signature

```rust
fn no_op() -> Command
```

## Docstring

`DeleteSelection` with nothing selected takes the `if !changed` early
return, so it is the cheapest command that reports
`changed == false`.

## Source
Lines 166–168 in `crates/oxide-engine/src/exec/batch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [batch](/crates/oxide-engine/src/exec/batch.md) |
| called_by | [a_batch_that_changes_nothing_records_no_entry_and_keeps_redo](/crates/oxide-engine/src/exec/batch/a_batch_that_changes_nothing_records_no_entry_and_keeps_redo.md) |
| called_by | [unchanged_commands_inside_a_batch_do_not_dilute_the_patch](/crates/oxide-engine/src/exec/batch/unchanged_commands_inside_a_batch_do_not_dilute_the_patch.md) |
