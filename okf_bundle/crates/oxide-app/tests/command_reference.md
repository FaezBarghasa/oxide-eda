---
okf_version: "0.2"
type: Module
title: command_reference
description: "Generates `docs/COMMANDS.md` from the command catalog and the shipped"
resource: crates/oxide-app/tests/command_reference.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/command_reference
language: rust
---

# command_reference

Generates `docs/COMMANDS.md` from the command catalog and the shipped

## Docstring

Generates `docs/COMMANDS.md` from the command catalog and the shipped
keymap profiles, and fails if the checked-in copy has drifted (#277).

There was no command reference at all before this.
`docs/KEYBOARD_SHORTCUTS.md` documents the TOML profile *format* and
contains no command table, so a user could not discover what they were
allowed to bind a key to except by reading `keymap/catalog/*.rs`
(oxide#517 §8).

Generated rather than hand-written for the reason the epic exists: the
catalog is the single source, and a hand-kept second copy drifts. Same
golden-file shape as `command_id_surface_matches_golden_snapshot`
(oxide#276) — regenerate deliberately, never just to make a test pass:

```text
UPDATE_DOCS=1 cargo test -p oxide-app --test command_reference
```

## Relationships

| Type | Target |
|------|--------|
| related | [bindings](/crates/oxide-app/tests/command_reference/bindings.md) |
| related | [cell](/crates/oxide-app/tests/command_reference/cell.md) |
| related | [render](/crates/oxide-app/tests/command_reference/render.md) |
| related | [command_reference_matches_the_catalog](/crates/oxide-app/tests/command_reference/command_reference_matches_the_catalog.md) |
| related | [both_shipped_profiles_yield_bindings](/crates/oxide-app/tests/command_reference/both_shipped_profiles_yield_bindings.md) |
