---
okf_version: "0.2"
type: Function
title: main
resource: tools/regen_wordmark_path.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:tools"
  - "domain:regen_wordmark_path.py"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:00Z"
concept_id: tools/regen_wordmark_path/main
language: python
---

# main

## Signature

```python
def main() -> int
```

## Returns
`int`

## Source
Lines 107–126 in `tools/regen_wordmark_path.py`

## Relationships

| Type | Target |
|------|--------|
| related | [regen_wordmark_path](/tools/regen_wordmark_path.md) |
| calls | [build_wordmark_path_d](/tools/regen_wordmark_path/build_wordmark_path_d.md) |
| calls | [read_text](/crates/oxide-library/src/search_index/read_text.md) |
| calls | [replace_path_d](/tools/regen_wordmark_path/replace_path_d.md) |
