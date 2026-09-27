---
okf_version: "0.2"
type: Function
title: redo
resource: crates/oxide-engine/src/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/history/redo
language: rust
---

# redo

## Signature

```rust
impl Engine { pub fn redo(&mut self) -> Result<Option<PatchPair>, EngineError> }
```

## Visibility

- `pub`

## Source
Lines 39–51 in `crates/oxide-engine/src/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-engine/src/history.md) |
