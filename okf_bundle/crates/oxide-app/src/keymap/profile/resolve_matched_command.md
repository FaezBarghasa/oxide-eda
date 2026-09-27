---
okf_version: "0.2"
type: Function
title: resolve_matched_command
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/resolve_matched_command
language: rust
---

# resolve_matched_command

## Signature

```rust
fn resolve_matched_command(
    matches: Vec<(bool, usize, usize, &CompiledBinding)>,
) -> Option<AppCommandId>
```

## Source
Lines 311–327 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [lookup](/crates/oxide-app/src/keymap/profile/lookup.md) |
