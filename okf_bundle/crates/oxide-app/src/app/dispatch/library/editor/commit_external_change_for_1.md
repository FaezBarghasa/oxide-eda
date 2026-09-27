---
okf_version: "0.2"
type: Function
title: commit_external_change_for
description: "Find the open library whose root contains `path`, then ask its"
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/commit_external_change_for_1
language: rust
---

# commit_external_change_for

Find the open library whose root contains `path`, then ask its

## Signature

```rust
fn commit_external_change_for(&self, path: &std::path::Path, message: &str)
```

## Docstring

Find the open library whose root contains `path`, then ask its
adapter to stage + commit. Best-effort: silently returns when
no mounted library covers `path` (lone-file edit) or when the
commit itself fails (warning is emitted via tracing). Never
blocks the user — the file write already succeeded.

## Source
Lines 526–550 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
