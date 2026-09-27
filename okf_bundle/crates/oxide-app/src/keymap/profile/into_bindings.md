---
okf_version: "0.2"
type: Function
title: into_bindings
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/into_bindings
language: rust
---

# into_bindings

## Signature

```rust
impl TomlKeymapSection { fn into_bindings(self) -> impl Iterator<Item = Result<ShortcutBinding, ProfileLoadError>> }
```

## Source
Lines 810–832 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [Command](/crates/oxide-engine/src/command/Command.md) |
