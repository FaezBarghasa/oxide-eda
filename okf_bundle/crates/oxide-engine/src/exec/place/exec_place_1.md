---
okf_version: "0.2"
type: Function
title: exec_place
resource: crates/oxide-engine/src/exec/place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/place/exec_place_1
language: rust
---

# exec_place

## Signature

```rust
pub(crate) fn exec_place(
        &mut self,
        before: SchematicSheet,
        cmd: Command,
    ) -> Result<CommandResult, EngineError>
```

## Visibility

- `pub(crate)`

## Source
Lines 21–227 in `crates/oxide-engine/src/exec/place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [place](/crates/oxide-engine/src/exec/place.md) |
