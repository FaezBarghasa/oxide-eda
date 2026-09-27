---
okf_version: "0.2"
type: Function
title: open_project_tree_document_snxsym_carries_the_task_out
description: "Regression (#99 part 1): `open_project_tree_document` used to"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/open_document/open_project_tree_document_snxsym_carries_the_task_out
language: rust
---

# open_project_tree_document_snxsym_carries_the_task_out

Regression (#99 part 1): `open_project_tree_document` used to

## Signature

```rust
fn open_project_tree_document_snxsym_carries_the_task_out()
```

## Decorators

- `test`

## Docstring

Regression (#99 part 1): `open_project_tree_document` used to
return `Result<()>` and the `.snxsym` / `.snxlib` branches
discarded the `Task` from `handle_open_primitive` /
`handle_open_library_browser` via `let _ = ...`. The signature
now carries the `Task` out — the explicit `Task<Message>`
annotation below is a compile-time tripwire: this test stops
compiling (rather than merely failing) if that return type is
ever swallowed back down to `Result<()>`.

A Task-*value* assertion isn't meaningful here: every callee on
this path returns `Task::none()` unconditionally today (async
`.snxlib` mounting lands in part 2 of #99), so the propagated
and dropped cases are runtime-indistinguishable until then.
[test]

## Source
Lines 242–265 in `crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open_document](/crates/oxide-app/src/app/handlers/dock/project_navigation/open_document.md) |
