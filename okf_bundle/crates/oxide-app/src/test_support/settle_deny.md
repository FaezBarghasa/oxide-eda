---
okf_version: "0.2"
type: Function
title: settle_deny
description: "`icacls`'s exit code is not proof the deny is enforced yet — under"
resource: crates/oxide-app/src/test_support.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/test_support/settle_deny
language: rust
---

# settle_deny

`icacls`'s exit code is not proof the deny is enforced yet — under

## Signature

```rust
fn settle_deny(dir: &Path)
```

## Decorators

- `cfg(windows)`

## Docstring

`icacls`'s exit code is not proof the deny is enforced yet — under
full-workspace parallel test load on Windows, the write to the
directory's security descriptor can lose the race against the very next
`File::create` in the same directory, so a caller that trusts the exit
code alone occasionally observes a write that should have failed
succeed instead (#482). Poll with a real probe write and only return
once the deny is actually observed to take effect.
[cfg(windows)]

## Source
Lines 74–86 in `crates/oxide-app/src/test_support.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [test_support](/crates/oxide-app/src/test_support.md) |
| called_by | [deny](/crates/oxide-app/src/test_support/deny.md) |
