---
okf_version: "0.2"
type: Function
title: handle_browser_edit_msg
description: "Apply a `BrowserEditMsg` to the active edit modal for `library_path`."
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_edit_msg_1
language: rust
---

# handle_browser_edit_msg

Apply a `BrowserEditMsg` to the active edit modal for `library_path`.

## Signature

```rust
pub(super) fn handle_browser_edit_msg(
        &mut self,
        library_path: std::path::PathBuf,
        msg: BrowserEditMsg,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Apply a `BrowserEditMsg` to the active edit modal for `library_path`.

## Source
Lines 406–602 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [hash_row_content](/crates/oxide-library/src/hash/hash_row_content.md) |
