---
okf_version: "0.2"
type: Function
title: attach_parked_schematic_tab
description: "Reattach a tab to a schematic engine that's already parked in"
resource: crates/oxide-app/src/app/load_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/load_gateway/attach_parked_schematic_tab
language: rust
---

# attach_parked_schematic_tab

Reattach a tab to a schematic engine that's already parked in

## Signature

```rust
impl Oxide { pub(crate) fn attach_parked_schematic_tab(&mut self, path: PathBuf, title: String) }
```

## Visibility

- `pub(crate)`

## Docstring

Reattach a tab to a schematic engine that's already parked in
`document_state.engines`. Used when the user reopens a file
that was closed while dirty — we kept the engine alive in
`close_tab_now` precisely so the in-memory edits survive the
reopen. Re-parsing from disk would discard those edits.

Pre-condition: `document_state.engines.contains_key(&path)`.
Post-condition: a new tab exists pointing at `path` with
`dirty: true`, the active engine is the parked entry, and the
canvas reflects the parked sheet's current state.

## Source
Lines 246–269 in `crates/oxide-app/src/app/load_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [load_gateway](/crates/oxide-app/src/app/load_gateway.md) |
