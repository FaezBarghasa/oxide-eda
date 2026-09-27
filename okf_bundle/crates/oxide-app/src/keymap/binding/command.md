---
okf_version: "0.2"
type: Function
title: command
resource: crates/oxide-app/src/keymap/binding.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/binding/command
language: rust
---

# command

## Signature

```rust
impl ShortcutBindingAction { pub fn command(&self) -> Option<&AppCommandId> }
```

## Visibility

- `pub`

## Source
Lines 305–310 in `crates/oxide-app/src/keymap/binding.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [binding](/crates/oxide-app/src/keymap/binding.md) |
