---
okf_version: "0.2"
type: Function
title: read_first_run_tour_dismissed
description: "Has the user dismissed the first-run tour card? Default `false` so a"
resource: crates/oxide-app/src/fonts/misc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/fonts/misc/read_first_run_tour_dismissed
language: rust
---

# read_first_run_tour_dismissed

Has the user dismissed the first-run tour card? Default `false` so a

## Signature

```rust
pub fn read_first_run_tour_dismissed() -> bool
```

## Visibility

- `pub`

## Docstring

Has the user dismissed the first-run tour card? Default `false` so a
fresh install shows the card on first launch; once dismissed (via the
X button, Esc, or any canvas interaction) the flag flips to `true`
and stays that way for the lifetime of the prefs file.

## Source
Lines 13–22 in `crates/oxide-app/src/fonts/misc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [misc](/crates/oxide-app/src/fonts/misc.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
