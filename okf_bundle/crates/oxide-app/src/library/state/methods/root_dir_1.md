---
okf_version: "0.2"
type: Function
title: root_dir
description: "Directory holding the `.snxlib` file — the per-library git"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/root_dir_1
language: rust
---

# root_dir

Directory holding the `.snxlib` file — the per-library git

## Signature

```rust
pub fn root_dir(&self) -> Option<&Path>
```

## Visibility

- `pub`

## Docstring

Directory holding the `.snxlib` file — the per-library git
working tree and parent of `symbols/` / `footprints/` /
`sims/`. Returns `None` only when `root` is rooted (no
parent), which shouldn't happen for legitimate libraries
since `.snxlib` always lives inside its parent dir.

Use this whenever you need to compare paths against the
library's working tree (e.g. "is this `.snxsym` inside this
library?") or join sibling paths
(`root_dir().join("symbols")`).

## Source
Lines 450–452 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
