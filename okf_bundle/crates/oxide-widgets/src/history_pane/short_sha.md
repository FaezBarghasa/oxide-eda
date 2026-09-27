---
okf_version: "0.2"
type: Function
title: short_sha
description: "Trim a SHA to the seven-char shorthand `git log --oneline` uses."
resource: crates/oxide-widgets/src/history_pane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/history_pane/short_sha
language: rust
---

# short_sha

Trim a SHA to the seven-char shorthand `git log --oneline` uses.

## Signature

```rust
fn short_sha(sha: &str) -> String
```

## Docstring

Trim a SHA to the seven-char shorthand `git log --oneline` uses.
Anything shorter than 7 chars (e.g. a test stub) is returned
untouched. LO-4 considered: returning `&str` here would force the
iced `text()` widget to capture the entry's lifetime, which doesn't
compose with the row-builder iterator's per-item ownership. The
allocation is paid anyway when the widget materialises the string.

## Source
Lines 155–161 in `crates/oxide-widgets/src/history_pane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history_pane](/crates/oxide-widgets/src/history_pane.md) |
