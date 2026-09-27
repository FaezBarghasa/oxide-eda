---
okf_version: "0.2"
type: Function
title: classify_bump_distinguishes_three_buckets
description: "`classify_bump` distinguishes patch / minor / major by the"
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/classify_bump_distinguishes_three_buckets
language: rust
---

# classify_bump_distinguishes_three_buckets

`classify_bump` distinguishes patch / minor / major by the

## Signature

```rust
fn classify_bump_distinguishes_three_buckets()
```

## Decorators

- `test`

## Docstring

`classify_bump` distinguishes patch / minor / major by the
leading two numeric segments. Anything that fails to parse
upgrades to Major — defensive default since mismatched-format
pairs likely indicate breaking schema drift.
[test]

## Source
Lines 395–403 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
