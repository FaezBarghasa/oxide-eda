---
okf_version: "0.2"
type: Function
title: dir
description: "The project's directory — the *one* convention for \"where this"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/dir
language: rust
---

# dir

The project's directory — the *one* convention for "where this

## Signature

```rust
impl LoadedProject { pub fn dir(&self) -> &std::path::Path }
```

## Visibility

- `pub`

## Docstring

The project's directory — the *one* convention for "where this
project's relative sheet filenames resolve from".

Two conventions used to coexist: the parent of the `.snxprj` on disk,
and the persisted `data.dir` string. The loader patches `data.dir` to
the opened path's parent, so they agree for a freshly-loaded project —
but a `LoadedProject` assembled any other way (project creation, a
`.snxprj` that recorded an absolute `dir` that no longer matches) can
have them disagree, and then ownership resolution succeeds while the
sheet paths it produces do not exist. The export skips those pages
silently. `path.parent()` is the only one that cannot be stale: it is
derived from the file actually opened.

## Source
Lines 285–289 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
