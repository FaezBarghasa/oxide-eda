---
okf_version: "0.2"
type: Function
title: exec_edits
resource: crates/oxide-engine/src/exec/edits.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-engine/src/exec/edits/exec_edits
language: rust
---

# exec_edits

## Signature

```rust
impl Engine { pub(crate) fn exec_edits(
        &mut self,
        before: SchematicSheet,
        cmd: Command,
    ) -> Result<CommandResult, EngineError> }
```

## Visibility

- `pub(crate)`

## Source
Lines 6–366 in `crates/oxide-engine/src/exec/edits.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edits](/crates/oxide-engine/src/exec/edits.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
