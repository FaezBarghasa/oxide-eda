---
okf_version: "0.2"
type: Function
title: register_standalone_library_on_project
description: "Find the project containing `path` and push a `LibraryEntry`"
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/register_standalone_library_on_project_1
language: rust
---

# register_standalone_library_on_project

Find the project containing `path` and push a `LibraryEntry`

## Signature

```rust
fn register_standalone_library_on_project(&mut self, path: &std::path::Path)
```

## Docstring

Find the project containing `path` and push a `LibraryEntry`
for it onto `data.libraries` (project-local relative path when
`path` is inside the project dir, absolute otherwise). Marks
the project dirty + refreshes the panel context so the new
entry shows immediately. No-op when the path is already
registered, or when no loaded project owns the file's parent.

## Source
Lines 377–431 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
