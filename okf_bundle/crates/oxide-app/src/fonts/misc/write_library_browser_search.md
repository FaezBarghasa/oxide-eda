---
okf_version: "0.2"
type: Function
title: write_library_browser_search
description: "Persist a single library's search query. Reading the existing map"
resource: crates/oxide-app/src/fonts/misc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/misc/write_library_browser_search
language: rust
---

# write_library_browser_search

Persist a single library's search query. Reading the existing map

## Signature

```rust
pub fn write_library_browser_search(library_path: &std::path::Path, query: &str)
```

## Visibility

- `pub`

## Docstring

Persist a single library's search query. Reading the existing map
from disk first means concurrent updates to other libraries don't
stomp each other (same-process only — cross-process serialisation
is out of scope for prefs).

## Source
Lines 95–109 in `crates/oxide-app/src/fonts/misc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [misc](/crates/oxide-app/src/fonts/misc.md) |
| calls | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [handle_browser_search_changed](/crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_search_changed.md) |
