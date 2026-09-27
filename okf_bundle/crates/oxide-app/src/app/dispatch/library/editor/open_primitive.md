---
okf_version: "0.2"
type: Function
title: open_primitive
description: "Open a `.snxsym` / `.snxfpt` tab, or say why it could not be"
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/open_primitive
language: rust
---

# open_primitive

Open a `.snxsym` / `.snxfpt` tab, or say why it could not be

## Signature

```rust
impl Oxide { fn open_primitive(&mut self, path: std::path::PathBuf) -> anyhow::Result<Task<Message>> }
```

## Docstring

Open a `.snxsym` / `.snxfpt` tab, or say why it could not be
opened. Never surfaces anything itself — see the boundary above.

## Source
Lines 60–199 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
