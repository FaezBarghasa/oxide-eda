---
okf_version: "0.2"
type: Function
title: dispatch_tool_message
resource: crates/oxide-app/src/app/dispatch/tool.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/tool/dispatch_tool_message
language: rust
---

# dispatch_tool_message

## Signature

```rust
impl Oxide { pub(super) fn dispatch_tool_message(&mut self, message: ToolMessage) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Source
Lines 157–492 in `crates/oxide-app/src/app/dispatch/tool.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool](/crates/oxide-app/src/app/dispatch/tool.md) |
