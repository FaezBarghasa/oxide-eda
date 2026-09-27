---
okf_version: "0.2"
type: Function
title: empty_pcb_board
description: "Minimal but valid `PcbBoard` fixture — same shape as"
resource: crates/oxide-app/tests/regression/file_open_async.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/file_open_async/empty_pcb_board
language: rust
---

# empty_pcb_board

Minimal but valid `PcbBoard` fixture — same shape as

## Signature

```rust
fn empty_pcb_board() -> oxide_types::pcb::PcbBoard
```

## Docstring

Minimal but valid `PcbBoard` fixture — same shape as
`oxide_types::format::tests::empty_board`.

## Source
Lines 221–238 in `crates/oxide-app/tests/regression/file_open_async.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [file_open_async](/crates/oxide-app/tests/regression/file_open_async.md) |
| called_by | [opening_the_same_pcb_twice_before_it_completes_spawns_only_one_task](/crates/oxide-app/tests/regression/file_open_async/opening_the_same_pcb_twice_before_it_completes_spawns_only_one_task.md) |
| called_by | [pcb_open_finished_ok_opens_the_tab_like_the_old_sync_path_did](/crates/oxide-app/tests/regression/file_open_async/pcb_open_finished_ok_opens_the_tab_like_the_old_sync_path_did.md) |
