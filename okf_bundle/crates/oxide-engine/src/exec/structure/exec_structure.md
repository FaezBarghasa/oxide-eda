---
okf_version: "0.2"
type: Function
title: exec_structure
resource: crates/oxide-engine/src/exec/structure.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-engine/src/exec/structure/exec_structure
language: rust
---

# exec_structure

## Signature

```rust
impl Engine { pub(crate) fn exec_structure(
        &mut self,
        before: SchematicSheet,
        cmd: Command,
    ) -> Result<CommandResult, EngineError> }
```

## Visibility

- `pub(crate)`

## Source
Lines 6–400 in `crates/oxide-engine/src/exec/structure.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [structure](/crates/oxide-engine/src/exec/structure.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [reconcile_child_sheet_pins](/crates/oxide-engine/src/sheet/reconcile_child_sheet_pins.md) |
