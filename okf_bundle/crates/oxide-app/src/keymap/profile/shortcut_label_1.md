---
okf_version: "0.2"
type: Function
title: shortcut_label
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/shortcut_label_1
language: rust
---

# shortcut_label

## Signature

```rust
pub fn shortcut_label(&self, command: &AppCommandId) -> Option<String>
```

## Visibility

- `pub`

## Source
Lines 233–252 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
