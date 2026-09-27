---
okf_version: "0.2"
type: Function
title: dispatch_project_message
description: "Project lifecycle family handler (namespaced, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/overlay/dispatch_project_message_1
language: rust
---

# dispatch_project_message

Project lifecycle family handler (namespaced, ADR-0001 D3).

## Signature

```rust
pub(crate) fn dispatch_project_message(&mut self, msg: ProjectMsg) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

Project lifecycle family handler (namespaced, ADR-0001 D3).
Covers the project-close / app-quit confirm modals, the Project
Options dismiss, the Add-Existing / Add-New-Schematic file-picker
completions, and the async git-commit completion.

## Source
Lines 68–93 in `crates/oxide-app/src/app/dispatch/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/app/dispatch/overlay.md) |
