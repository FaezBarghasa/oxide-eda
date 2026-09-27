---
okf_version: "0.2"
type: Function
title: bindings
description: "Command id → every trigger bound to it in one profile, in file order."
resource: crates/oxide-app/tests/command_reference.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/command_reference/bindings
language: rust
---

# bindings

Command id → every trigger bound to it in one profile, in file order.

## Signature

```rust
fn bindings(profile: &str) -> BTreeMap<&str, Vec<&str>>
```

## Docstring

Command id → every trigger bound to it in one profile, in file order.

Parses the `"TRIGGER" = "command_id"` binding lines directly rather
than going through the TOML loader: this is a documentation artifact,
and reading the shipped file verbatim is what keeps it honest. Profile
metadata (`profile_id = "altium"`, `context = "global"`) is excluded
by requiring a quoted key.

## Source
Lines 35–55 in `crates/oxide-app/tests/command_reference.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_reference](/crates/oxide-app/tests/command_reference.md) |
| called_by | [both_shipped_profiles_yield_bindings](/crates/oxide-app/tests/command_reference/both_shipped_profiles_yield_bindings.md) |
| called_by | [render](/crates/oxide-app/tests/command_reference/render.md) |
