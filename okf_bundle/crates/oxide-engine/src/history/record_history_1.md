---
okf_version: "0.2"
type: Function
title: record_history
resource: crates/oxide-engine/src/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/history/record_history_1
language: rust
---

# record_history

## Signature

```rust
pub(crate) fn record_history(&mut self, snapshot: SchematicSheet, patch_pair: PatchPair)
```

## Visibility

- `pub(crate)`

## Source
Lines 53–73 in `crates/oxide-engine/src/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-engine/src/history.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
