---
okf_version: "0.2"
type: Function
title: handle_library_updates_skip_all
description: User clicked Skip All — close the modal and record the path on
resource: crates/oxide-app/src/app/dispatch/library/updates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/updates/handle_library_updates_skip_all
language: rust
---

# handle_library_updates_skip_all

User clicked Skip All — close the modal and record the path on

## Signature

```rust
impl Oxide { pub(super) fn handle_library_updates_skip_all(&mut self) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

User clicked Skip All — close the modal and record the path on
`skipped_updates_for` so the status bar can flag it persistently.

## Source
Lines 28–35 in `crates/oxide-app/src/app/dispatch/library/updates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/app/dispatch/library/updates.md) |
