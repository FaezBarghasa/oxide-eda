---
okf_version: "0.2"
type: Function
title: save_at_leaves_original_intact_when_write_fails
description: "`save_at` must go through `atomic_write`, not `fs::write`: a failed"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/save_at_leaves_original_intact_when_write_fails
language: rust
---

# save_at_leaves_original_intact_when_write_fails

`save_at` must go through `atomic_write`, not `fs::write`: a failed

## Signature

```rust
fn save_at_leaves_original_intact_when_write_fails()
```

## Decorators

- `test`

## Docstring

`save_at` must go through `atomic_write`, not `fs::write`: a failed
save leaves the previously persisted list fully intact.

Discriminator: denying new-file creation in the destination's
parent directory makes `atomic_write`'s `File::create(&tmp)` fail
before it can touch the destination, regardless of the unique
per-writer temp name it picks (#416). A plain `fs::write` would
ignore that and clobber the old file — so this test fails on a
revert.
[test]

## Source
Lines 232–253 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [save_at](/crates/oxide-app/src/panels/components_panel/global_prefs/save_at.md) |
