---
okf_version: "0.2"
type: Function
title: lookup
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/lookup_1
language: rust
---

# lookup

## Signature

```rust
pub fn lookup(&self, input: &[KeyStroke], contexts: &[ShortcutContext]) -> KeyLookup
```

## Visibility

- `pub`

## Source
Lines 206–231 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [context_depth](/crates/oxide-app/src/keymap/profile/context_depth.md) |
| calls | [resolve_matched_command](/crates/oxide-app/src/keymap/profile/resolve_matched_command.md) |
