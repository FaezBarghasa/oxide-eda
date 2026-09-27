---
okf_version: "0.2"
type: Function
title: handle_preferences_navigation_requested
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_navigation_requested_1
language: rust
---

# handle_preferences_navigation_requested

## Signature

```rust
pub(crate) fn handle_preferences_navigation_requested(
        &mut self,
        nav: crate::preferences::PrefNav,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 131–137 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
