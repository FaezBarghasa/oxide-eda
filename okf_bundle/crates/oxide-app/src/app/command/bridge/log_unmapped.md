---
okf_version: "0.2"
type: Function
title: log_unmapped
description: Report an id that reached the bridge and found no arm.
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/log_unmapped
language: rust
---

# log_unmapped

Report an id that reached the bridge and found no arm.

## Signature

```rust
fn log_unmapped(command: &AppCommandId)
```

## Docstring

Report an id that reached the bridge and found no arm.

Lives here, not at a call site, so every consumer of the registry
inherits it — the keyboard today, and the menu bar, command palette
and CLI as they are rewired onto `dispatch_command` (#367, #366).

The two failure modes want different words because they need
different fixes:
- a real catalog id with no arm — the binding is advertised in the
Keyboard Shortcuts pane and does nothing. 64 ids are in this state;
see `tests::UNMAPPED_CATALOG_IDS`.
- an id that is in no catalog at all — almost always a typo in a
user-edited keymap TOML, which `keymap::profile` accepts without
ever validating against the catalog.

Deliberately `log_warning`, not `log_error`: neither case loses data
or leaves the app wrong, and neither is actionable mid-edit.

## Source
Lines 174–187 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| called_by | [core_to_message](/crates/oxide-app/src/app/command/bridge/core_to_message.md) |
