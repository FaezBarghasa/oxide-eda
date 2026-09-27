---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/new
language: rust
---

# new

## Signature

```rust
impl KeymapRecorderState { pub fn new(
        command: crate::keymap::AppCommandId,
        command_label: String,
        context: crate::keymap::ShortcutContext,
        trigger: String,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 47–72 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
