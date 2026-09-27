---
okf_version: "0.2"
type: Function
title: command_id_surface_matches_golden_snapshot
description: "Golden-snapshot test (oxide#276): locks the STABLE command-id"
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/command_id_surface_matches_golden_snapshot
language: rust
---

# command_id_surface_matches_golden_snapshot

Golden-snapshot test (oxide#276): locks the STABLE command-id

## Signature

```rust
fn command_id_surface_matches_golden_snapshot()
```

## Decorators

- `test`

## Docstring

Golden-snapshot test (oxide#276): locks the STABLE command-id
surface — `id` + `group` + `category` — because external CLI/plugin
tooling depends on ids staying put. The churny descriptor fields
(`icon`/`keybind`/`enable`/`flags`, added by #275/#479) are
deliberately NOT captured here and may change freely.

A failing diff means one of two things:
- a command was renamed or removed. This is a breaking change for
downstream consumers: add an alias + a deprecation note, do NOT
just regenerate the golden to make the test pass again.
- a command was added, or an existing one's `group`/`category`
changed on purpose. Regenerate with:
`UPDATE_GOLDEN=1 cargo test -p oxide-app command_id_surface_matches_golden_snapshot`
[test]

## Source
Lines 415–448 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
| calls | [all_metadata](/crates/oxide-app/src/keymap/catalog/mod/all_metadata.md) |
