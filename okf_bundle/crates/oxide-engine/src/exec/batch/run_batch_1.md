---
okf_version: "0.2"
type: Function
title: run_batch
description: "The suppressed half of [`Engine::execute_batch`]."
resource: crates/oxide-engine/src/exec/batch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/batch/run_batch_1
language: rust
---

# run_batch

The suppressed half of [`Engine::execute_batch`].

## Signature

```rust
fn run_batch(
        &mut self,
        commands: impl IntoIterator<Item = Command>,
    ) -> Result<Option<PatchPair>, EngineError>
```

## Docstring

The suppressed half of [`Engine::execute_batch`].

Returns the coalesced patch, or `None` when no command in the batch
changed anything.

## Source
Lines 94–124 in `crates/oxide-engine/src/exec/batch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [batch](/crates/oxide-engine/src/exec/batch.md) |
