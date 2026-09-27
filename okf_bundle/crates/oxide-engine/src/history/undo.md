---
okf_version: "0.2"
type: Function
title: undo
resource: crates/oxide-engine/src/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/history/undo
language: rust
---

# undo

## Signature

```rust
impl Engine { pub fn undo(&mut self) -> Result<Option<PatchPair>, EngineError> }
```

## Visibility

- `pub`

## Source
Lines 25–37 in `crates/oxide-engine/src/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-engine/src/history.md) |
