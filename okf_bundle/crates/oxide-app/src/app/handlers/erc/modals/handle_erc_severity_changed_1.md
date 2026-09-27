---
okf_version: "0.2"
type: Function
title: handle_erc_severity_changed
resource: crates/oxide-app/src/app/handlers/erc/modals.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/modals/handle_erc_severity_changed_1
language: rust
---

# handle_erc_severity_changed

## Signature

```rust
pub(crate) fn handle_erc_severity_changed(
        &mut self,
        rule: oxide_erc::RuleKind,
        severity: oxide_erc::Severity,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 19–34 in `crates/oxide-app/src/app/handlers/erc/modals.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [modals](/crates/oxide-app/src/app/handlers/erc/modals.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [write_erc_severity_overrides](/crates/oxide-app/src/fonts/erc/write_erc_severity_overrides.md) |
