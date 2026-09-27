---
okf_version: "0.2"
type: Function
title: read_library_browser_searches
description: "Read the persisted per-`.snxlib` Library Browser search queries."
resource: crates/oxide-app/src/fonts/misc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/misc/read_library_browser_searches
language: rust
---

# read_library_browser_searches

Read the persisted per-`.snxlib` Library Browser search queries.

## Signature

```rust
pub fn read_library_browser_searches() -> std::collections::HashMap<PathBuf, String>
```

## Visibility

- `pub`

## Docstring

Read the persisted per-`.snxlib` Library Browser search queries.
Keyed by the absolute path's display string; entries for libraries
that no longer exist on disk are harmless — they're loaded but only
touched again when the user reopens that library.

## Source
Lines 66–89 in `crates/oxide-app/src/fonts/misc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [misc](/crates/oxide-app/src/fonts/misc.md) |
| called_by | [finish_open_library_browser](/crates/oxide-app/src/app/dispatch/library/browser/mod/finish_open_library_browser.md) |
