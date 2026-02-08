---
name: skill__probe__outline
description: probe outline `probe outline <FILE> [--format text|json] [--kind function|struct|impl|trait|type|enum|class]` 列出文件顶层 symbol
---

**Command:**
```bash
probe outline <FILE> [--format text|json] [--kind <TYPE>]
```

**Parameters:**
| Parameter | Description | Example |
|-----------|-------------|---------|
| `<FILE>` | 文件路径 | `src/main.rs` |
| `--format` | text 或 json | `--format json` |
| `--kind` | 过滤类型 | `--kind function` |

**Output (text):**
```
struct   ExtractOptions                                     src/extract/mod.rs:38
function handle_extract                                     src/extract/mod.rs:342

💡 Use `mod.rs:line` or `mod.rs#symbol` to extract code
```

**Output (json):**
```json
[
  {"name": "ExtractOptions", "type": "struct", "line": 38, "signature": "..."},
  {"name": "handle_extract", "type": "function", "line": 342, "signature": "..."}
]
```

**Examples:**
```bash
probe outline src/main.rs
probe outline src/main.rs --format json
probe outline src/main.rs --kind function
```
