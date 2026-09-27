---
okf_version: "0.2"
type: Function
title: load_for_update
description: "Load `prefs.json` as the object it is supposed to be, or say why not."
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/load_for_update
language: rust
---

# load_for_update

Load `prefs.json` as the object it is supposed to be, or say why not.

## Signature

```rust
fn load_for_update(
    path: &Path,
) -> Result<serde_json::Map<String, serde_json::Value>, PrefsLoadError>
```

## Docstring

Load `prefs.json` as the object it is supposed to be, or say why not.

Only two inputs yield an empty map, and both genuinely carry no user
data to protect: the file is absent (fresh install — this is the path
that must keep working), or it is empty / all ASCII whitespace, which
is what a writer killed between `File::create` and `write_all` leaves
behind.

Returning the `Map` rather than the enclosing `Value` makes "the
prefs root is an object" a fact the type carries, which is what lets
the `library_browser_searches` writer drop its
`.as_object_mut().expect(…)` panic.

## Source
Lines 99–115 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| called_by | [check_prefs_file_at](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at.md) |
| called_by | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
