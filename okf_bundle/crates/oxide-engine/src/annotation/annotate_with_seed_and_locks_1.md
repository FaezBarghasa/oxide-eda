---
okf_version: "0.2"
type: Function
title: annotate_with_seed_and_locks
description: "Same as `annotate_with_seed`, but skips every symbol whose uuid"
resource: crates/oxide-engine/src/annotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/annotation/annotate_with_seed_and_locks_1
language: rust
---

# annotate_with_seed_and_locks

Same as `annotate_with_seed`, but skips every symbol whose uuid

## Signature

```rust
pub fn annotate_with_seed_and_locks(
        &mut self,
        mode: AnnotateMode,
        next_by_prefix: &mut std::collections::HashMap<String, u32>,
        locked: &std::collections::HashSet<uuid::Uuid>,
    ) -> Result<bool, EngineError>
```

## Visibility

- `pub`

## Docstring

Same as `annotate_with_seed`, but skips every symbol whose uuid
appears in `locked`. Used by the Annotate dialog's per-row lock
checkboxes so the user can exclude individual designators from
reannotation (Altium's "Lock" column behaviour).

## Source
Lines 24–135 in `crates/oxide-engine/src/annotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotation](/crates/oxide-engine/src/annotation.md) |
