---
name: skill__probe__extract
description: probe extract `probe extract <FILE>#<symbolName>|<FILE>:<line> [--format <FORMAT>] [--context N]` 提取代码块
---

**Command:**
```bash
probe extract <FILE>#<symbolName>           # 提取指定符号
probe extract <FILE>:<line>               # 提取行号所在函数
probe extract <FILE>                      # 提取整个文件
```

**Parameters:**
| Parameter | Description | Example |
|-----------|-------------|---------|
| `<FILE>#<symbolName>` | 符号名称 | `src/main.rs#handle_extract` |
| `<FILE>:<line>` | 行号 | `src/main.rs:69` |
| `--format <FORMAT>` | 输出格式 | `color` `json` `xml` `markdown` |
| `--context N` | 上下文行数 | `--context 3` |

**File Spec Syntax:**
| Syntax | Meaning | Example |
|--------|---------|---------|
| `file.rs` | 整个文件 | `probe extract src/main.rs` |
| `file.rs#symbol` | 指定 symbol | `probe extract src/main.rs#handle_extract` |
| `file.rs:line` | 行号所在函数 | `probe extract src/main.rs:69` |
| `file.rs:start-end` | 行范围 | `probe extract src/main.rs:10-50` |

**Output (color):** 带语法高亮的代码块

**Output (json):**
```json
{
  "file": "src/main.rs",
  "symbol": "handle_extract",
  "line": 69,
  "end_line": 120,
  "code": "pub fn handle_extract(options: ExtractOptions) -> Result<()> { ... }"
}
```

**Examples:**
```bash
probe extract src/main.rs
probe extract src/main.rs#handle_extract --context 3
probe extract src/main.rs:69
probe extract src/main.rs --format json
```
