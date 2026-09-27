---
okf_version: "0.2"
type: Function
title: finish_open_library_browser
description: Steps 2 and 3 of opening a Library Browser tab — seed the
resource: crates/oxide-app/src/app/dispatch/library/browser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/mod/finish_open_library_browser
language: rust
---

# finish_open_library_browser

Steps 2 and 3 of opening a Library Browser tab — seed the

## Signature

```rust
impl Oxide { pub(super) fn finish_open_library_browser(
        &mut self,
        path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Steps 2 and 3 of opening a Library Browser tab — seed the
per-browser state, then activate or push the tab.

Split out of [`Self::handle_open_library_browser`] so the mount
completion handler (`handle_mount_finished`) can finish the
gesture without re-running the mount. Pure code motion: the body
below is what ran inline before #99 part 2c, in the same order.

## Source
Lines 85–154 in `crates/oxide-app/src/app/dispatch/library/browser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [browser](/crates/oxide-app/src/app/dispatch/library/browser/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [read_library_browser_searches](/crates/oxide-app/src/fonts/misc/read_library_browser_searches.md) |
