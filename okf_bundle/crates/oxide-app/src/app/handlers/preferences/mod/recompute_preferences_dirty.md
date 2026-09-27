---
okf_version: "0.2"
type: Function
title: recompute_preferences_dirty
description: Recompute the cached dirty flag from the full unsaved-state
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/recompute_preferences_dirty
language: rust
---

# recompute_preferences_dirty

Recompute the cached dirty flag from the full unsaved-state

## Signature

```rust
impl Oxide { fn recompute_preferences_dirty(&mut self) }
```

## Docstring

Recompute the cached dirty flag from the full unsaved-state
predicate. Every Preferences draft mutation routes through this —
never assign `preferences_dirty` from an ad-hoc comparison; that
drift class is exactly what let imperative edits be clobbered back
to "clean" (review #308 finding 1).

## Source
Lines 127–129 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
