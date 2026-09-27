---
okf_version: "0.2"
type: Function
title: activate_or_skip_duplicate_open
description: "#478 review — dedup guard shared by `open_schematic_file` /"
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/activate_or_skip_duplicate_open
language: rust
---

# activate_or_skip_duplicate_open

#478 review — dedup guard shared by `open_schematic_file` /

## Signature

```rust
impl Oxide { fn activate_or_skip_duplicate_open(&mut self, path: &std::path::Path) -> bool }
```

## Docstring

#478 review — dedup guard shared by `open_schematic_file` /
`open_pcb_file`. Mirrors the activate-existing convention in
`handle_open_primitive`: if `path` already has a tab, activate
it. If it doesn't, but an async open for it is already in
flight (spawned by a previous call, not yet completed), do
nothing and let that completion create the tab — spawning a
second `Task::perform` for the same path would let both
completions push a tab aliasing one `document_state.engines`
entry, and closing one would orphan the other. Returns `true`
when the caller must stop and not spawn a new open `Task`.

## Source
Lines 263–273 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
