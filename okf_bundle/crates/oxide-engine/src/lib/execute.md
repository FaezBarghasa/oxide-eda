---
okf_version: "0.2"
type: Function
title: execute
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/execute
language: rust
---

# execute

## Signature

```rust
impl Engine { pub fn execute(&mut self, cmd: Command) -> Result<CommandResult, EngineError> }
```

## Visibility

- `pub`

## Source
Lines 108–142 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
