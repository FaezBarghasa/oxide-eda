---
okf_version: "0.2"
type: Function
title: execute_batch
description: "Execute `commands` as a single undoable step."
resource: crates/oxide-engine/src/exec/batch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/batch/execute_batch_1
language: rust
---

# execute_batch

Execute `commands` as a single undoable step.

## Signature

```rust
pub fn execute_batch(
        &mut self,
        commands: impl IntoIterator<Item = Command>,
    ) -> Result<CommandResult, EngineError>
```

## Visibility

- `pub`

## Docstring

Execute `commands` as a single undoable step.

One `HistoryEntry` is recorded for the whole batch: the snapshot is
the document as it stood before the first command, and the
`DocumentPatch` flags of every command that changed something are
OR-ed together, so one invalidation covers everything the batch
touched. Commands reporting `changed == false` contribute nothing; a
batch in which nothing changed records no entry at all and leaves the
redo stack intact, exactly as a single unchanged `execute` does.

`semantic` is widened to [`SemanticPatch::DocumentReplaced`] when the
batch mixes kinds, because no single variant describes the result.

On `Err` the batch aborts transactionally — the document is restored,
no history entry is written, and the redo stack is untouched, so the
engine is left exactly as it was before the call. The error carries
no accumulated patch, so a caller that repaints defensively after a
failure must invalidate [`DocumentPatch::FULL`].

Batches do not nest. A nested call would consume the outer batch's
history entry, so it trips a `debug_assert`.

No error is logged here. `oxide-engine` has no logging dependency,
and the `Result` is the observability: `unused_must_use` is denied
workspace-wide, so no caller can drop it silently.

## Source
Lines 56–88 in `crates/oxide-engine/src/exec/batch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [batch](/crates/oxide-engine/src/exec/batch.md) |
