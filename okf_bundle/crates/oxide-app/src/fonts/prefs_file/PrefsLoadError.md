---
okf_version: "0.2"
type: Class
title: PrefsLoadError
description: "Why an existing `prefs.json` could not be loaded for an in-place"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/PrefsLoadError
language: rust
---

# PrefsLoadError

Why an existing `prefs.json` could not be loaded for an in-place

## Signature

```rust
pub enum PrefsLoadError
```

## Decorators

- `derive(Debug)`

## Visibility

- `pub`

## Docstring

Why an existing `prefs.json` could not be loaded for an in-place
update. Each variant is a state the old chain silently turned into
an empty file; `Display` names what the user has to fix.

The `Display` strings are deliberately sentence *fragments* that
follow a path — "…/prefs.json is not valid JSON: …" — because both
consumers put the path in front of them: the refusal report below,
and the Preferences banner (#602).
[derive(Debug)]

## Source
Lines 52–56 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
