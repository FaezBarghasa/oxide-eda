---
okf_version: "0.2"
type: Function
title: write_pref_atomic
description: "MD-32: persist `bytes` to `path` atomically (tmp + rename via"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/write_pref_atomic
language: rust
---

# write_pref_atomic

MD-32: persist `bytes` to `path` atomically (tmp + rename via

## Signature

```rust
fn write_pref_atomic(path: &Path, bytes: &[u8], context: &str)
```

## Docstring

MD-32: persist `bytes` to `path` atomically (tmp + rename via
`oxide_types::atomic_io`) and report on failure, instead of the
`let _ = std::fs::write(...)` pattern that swallows disk-full /
permission errors silently. Every preferences write in this module
tree funnels through here, so this is the single place a failed write
can be seen from.

Reported at `Error` and not `debug!`: the default filter is
`LevelFilter::Info` (`crate::diagnostics`), so a `debug!` here never
reaches the user's Messages panel and the failure is invisible in
every shipped build — which is the whole class of bug #594 and #602
were about. The setting the user just changed did not persist and
will be gone at the next launch; that is not a `debug!`.

## Source
Lines 46–58 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| called_by | [migrate_legacy_prefs](/crates/oxide-app/src/fonts/mod/migrate_legacy_prefs.md) |
| called_by | [update_prefs_json](/crates/oxide-app/src/fonts/prefs_file/update_prefs_json.md) |
