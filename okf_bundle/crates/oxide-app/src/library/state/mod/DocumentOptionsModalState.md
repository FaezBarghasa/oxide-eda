---
okf_version: "0.2"
type: Class
title: DocumentOptionsModalState
description: State for the Tools ▸ Document Options modal — keyed by the
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/DocumentOptionsModalState
language: rust
---

# DocumentOptionsModalState

State for the Tools ▸ Document Options modal — keyed by the

## Signature

```rust
pub struct DocumentOptionsModalState
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

State for the Tools ▸ Document Options modal — keyed by the
containing `.snxlib` root path so the dispatcher knows which
`OpenLibrary.display` to mutate. Working draft + the modal's
scratch buffer. Apply on Save; discard on Cancel.
[derive(Debug, Clone)]

## Methods

- `library_path`
- `library_name`
- `draft`

## Source
Lines 568–572 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
