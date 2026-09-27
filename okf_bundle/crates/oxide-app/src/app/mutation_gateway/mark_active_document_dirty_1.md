---
okf_version: "0.2"
type: Function
title: mark_active_document_dirty
description: "Mark the active document as having unsaved edits, without running a"
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/mark_active_document_dirty_1
language: rust
---

# mark_active_document_dirty

Mark the active document as having unsaved edits, without running a

## Signature

```rust
pub(crate) fn mark_active_document_dirty(&mut self) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

Mark the active document as having unsaved edits, without running a
render invalidation.

For the one edit path that mutates the engine through a method that
is not a `Command` — `Engine::annotate_with_seed_and_locks`, which
records its own history entry — so it cannot go through
[`Oxide::apply_engine_command`] but still has to reach
`dirty_paths` (#585). Everything else should use the gateway, which
calls this as part of `finish_schematic_mutation`.

Returns `false` when there is no active schematic session to mark.

## Source
Lines 123–128 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
