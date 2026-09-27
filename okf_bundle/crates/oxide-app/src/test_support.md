---
okf_version: "0.2"
type: Module
title: test_support
description: "Test-only helpers for persistence tests (#416, #469, #482)."
resource: crates/oxide-app/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/test_support
language: rust
---

# test_support

Test-only helpers for persistence tests (#416, #469, #482).

## Docstring

Test-only helpers for persistence tests (#416, #469, #482).

`atomic_write` picks a unique `<pid>-<counter>` temp sibling name per
call, so a test can no longer force a write failure by pre-creating a
directory at a fixed `<path>.tmp` name (that name is never used). Deny
new-file creation in the destination's parent directory instead —
that fails `File::create` regardless of the exact temp name chosen.

On Windows, the deny is settled with a real probe write
([`settle_deny`]) before `DenyNewFiles::on` returns, rather than
trusting `icacls`'s exit code — see #482.

## Relationships

| Type | Target |
|------|--------|
| related | [DenyNewFiles](/crates/oxide-app/src/test_support/DenyNewFiles.md) |
| related | [on](/crates/oxide-app/src/test_support/on.md) |
| related | [on](/crates/oxide-app/src/test_support/on.md) |
| related | [drop](/crates/oxide-app/src/test_support/drop.md) |
| related | [drop](/crates/oxide-app/src/test_support/drop.md) |
| related | [deny](/crates/oxide-app/src/test_support/deny.md) |
| related | [allow](/crates/oxide-app/src/test_support/allow.md) |
| related | [deny](/crates/oxide-app/src/test_support/deny.md) |
| related | [settle_deny](/crates/oxide-app/src/test_support/settle_deny.md) |
| related | [allow](/crates/oxide-app/src/test_support/allow.md) |
| related | [has_stray_tmp](/crates/oxide-app/src/test_support/has_stray_tmp.md) |
