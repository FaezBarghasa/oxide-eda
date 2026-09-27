---
okf_version: "0.2"
type: Function
title: cascade_commit_message
description: Synthesise the git commit message for a cascade-driven row update.
resource: crates/oxide-library/src/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/cascade/cascade_commit_message
language: rust
---

# cascade_commit_message

Synthesise the git commit message for a cascade-driven row update.

## Signature

```rust
fn cascade_commit_message(
    kind: PrimitiveKindTag,
    primitive_uuid: Uuid,
    new_version: &str,
    row_id: &RowId,
) -> String
```

## Docstring

Synthesise the git commit message for a cascade-driven row update.

The plan §3.5.cascade calls for a single combined commit covering
the primitive save + every cascaded row update; for v0.9 simplicity
we emit one commit per row with a clearly-marked subject so the
audit trail still groups visually under the same primitive UUID.
A future polish pass can fold these into the parent save's commit.

## Source
Lines 225–236 in `crates/oxide-library/src/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-library/src/cascade.md) |
| called_by | [cascade_after_save](/crates/oxide-library/src/cascade/cascade_after_save.md) |
