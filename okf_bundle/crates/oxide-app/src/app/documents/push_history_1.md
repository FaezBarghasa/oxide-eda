---
okf_version: "0.2"
type: Function
title: push_history
description: v0.24 Phase 1 (Track B) — capture a snapshot of the
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/push_history_1
language: rust
---

# push_history

v0.24 Phase 1 (Track B) — capture a snapshot of the

## Signature

```rust
pub fn push_history(&mut self)
```

## Visibility

- `pub`

## Docstring

v0.24 Phase 1 (Track B) — capture a snapshot of the
canonical state ahead of a mutation. Caller invokes this
BEFORE the mutation runs; on `undo()` we swap the snapshot
back in. New mutations clear the redo stack so the history
stays a single timeline.

Phase 2 wiring: every `dispatch::library` arm that mutates
state should `push_history()` first, then run the mutation.
The dispatcher's existing `with_parts` closure is the
natural integration site.

## Source
Lines 498–505 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
