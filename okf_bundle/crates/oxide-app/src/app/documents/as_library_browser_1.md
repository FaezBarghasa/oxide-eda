---
okf_version: "0.2"
type: Function
title: as_library_browser
description: "`Some(path)` if this tab is a Library Browser. The path is the"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/as_library_browser_1
language: rust
---

# as_library_browser

`Some(path)` if this tab is a Library Browser. The path is the

## Signature

```rust
pub fn as_library_browser(&self) -> Option<&PathBuf>
```

## Visibility

- `pub`

## Docstring

`Some(path)` if this tab is a Library Browser. The path is the
`.snxlib` directory the browser is bound to.

## Source
Lines 83–88 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
