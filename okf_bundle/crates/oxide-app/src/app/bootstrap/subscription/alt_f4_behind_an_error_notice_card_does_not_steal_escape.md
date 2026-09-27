---
okf_version: "0.2"
type: Function
title: alt_f4_behind_an_error_notice_card_does_not_steal_escape
description: "The concrete reachable repro from round 4's review: export fails"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/alt_f4_behind_an_error_notice_card_does_not_steal_escape
language: rust
---

# alt_f4_behind_an_error_notice_card_does_not_steal_escape

The concrete reachable repro from round 4's review: export fails

## Signature

```rust
fn alt_f4_behind_an_error_notice_card_does_not_steal_escape()
```

## Decorators

- `test`

## Docstring

The concrete reachable repro from round 4's review: export fails
(`error_notice_open` set, only the error card paints, per
`has_blocking_modal`'s early return), then the user hits Alt+F4 /
the OS close button with dirty documents, which sets
`app_quit_confirm_open` with no regard for what's currently
painted. Esc must dismiss the card the user can actually see, not
silently cancel a quit gate they never saw — leaving them stuck
staring at an export-error card that Esc appears to do nothing to.
[test]

## Source
Lines 1153–1163 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
