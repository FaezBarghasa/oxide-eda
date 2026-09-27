---
okf_version: "0.2"
type: Class
title: HistoryTarget
description: "What the active tab resolves to for the History panel. `Tracked`"
resource: crates/oxide-app/src/app/runtime/history.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/history/HistoryTarget
language: rust
---

# HistoryTarget

What the active tab resolves to for the History panel. `Tracked`

## Signature

```rust
enum HistoryTarget
```

## Docstring

What the active tab resolves to for the History panel. `Tracked`
means we found a `.git/` ancestor and have a relative pathspec
to walk; `Untracked` means we have an on-disk file but no
`.git/` was found (the user hasn't enabled version control on
this project yet); `None` means the active tab has no
addressable file (no tabs at all, or a ComponentEditor tab).

## Methods

- `project_dir`
- `rel_path`
- `full_path`
- `full_path`

## Source
Lines 112–121 in `crates/oxide-app/src/app/runtime/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/app/runtime/history.md) |
