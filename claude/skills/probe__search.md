---
name: skill__probe__search
description: probe search `probe search <QUERY> [<PATH>...] [options]` 语义代码搜索，支持 Elastic 查询语法
---

**Command:**
```bash
probe search <QUERY> [<PATH>...] [options]
```

**Parameters:**

| Parameter | Description | Example |
|-----------|-------------|---------|
| `<QUERY>` | 搜索模式（支持 regex 和 AND/OR） | `"handler AND ext:rs"` |
| `[<PATH>]` | 文件/目录路径 | `.` `src/` `file.rs` |
| `-f, --files-only` | 仅输出匹配文件（跳过 AST 解析） | `--files-only` |
| `-I, --ignore` | 自定义忽略模式（额外） | `--ignore "**/node_modules"` |
| `-n, --exclude-filenames` | 排除文件名匹配 | `--exclude-filenames` |
| `-r, --reranker` | 排序算法 | `--reranker hybrid` |
| `-s, --frequency` | 频率搜索（默认启用） | `--frequency false` |
| `-e, --exact` | 精确搜索（禁用分词） | `--exact` |
| `--strict-elastic-syntax` | 严格 Elastic 语法 | `--strict-elastic-syntax` |
| `-l, --language` | 限制编程语言 | `--language rust` |
| `--max-results` | 最大结果数量 | `--max-results 10` |
| `--max-bytes` | 最大字节限制 | `--max-bytes 10240` |
| `--max-tokens` | 最大 token 限制 | `--max-tokens 8192` |
| `--allow-tests` | 允许测试文件 | `--allow-tests` |
| `--no-gitignore` | 忽略 .gitignore | `--no-gitignore` |
| `--no-merge` | 禁用代码块合并 | `--no-merge` |
| `--merge-threshold` | 合并阈值（行数） | `--merge-threshold 10` |
| `--dry-run` | 仅输出文件名和行号 | `--dry-run` |
| `-o, --format` | 输出格式 | `--format json` |
| `--session` | 会话缓存 ID | `--session search-001` |
| `--timeout` | 超时秒数（默认 30） | `--timeout 60` |
| `--question` | 自然语言问题（BERT 重排序） | `--question "How does auth work?"` |
| `-v, --verbose` | 详细输出（显示选项和时间） | `-v` |

**Reranker Options:**
| Value | Description |
|-------|-------------|
| `bm25` | BM25 排序（默认） |
| `hybrid` | BM25 + 频率加权 |
| `hybrid2` | 改进的混合排序 |
| `tfidf` | TF-IDF 排序 |
| `ms-marco-tinybert` | TinyBERT 重排序（需 `--features bert-reranker`） |
| `ms-marco-minilm-l6` | MiniLM-L6 重排序（需 `--features bert-reranker`） |
| `ms-marco-minilm-l12` | MiniLM-L12 重排序（需 `--features bert-reranker`） |

**Format Options:**
| Value | Description |
|-------|-------------|
| `terminal` | 终端友好格式 |
| `markdown` | Markdown 格式 |
| `plain` | 纯文本格式 |
| `color` | 带语法高亮（默认） |
| `outline` | 代码大纲格式（默认） |
| `json` | JSON 格式（机器可读） |
| `xml` | XML 格式（机器可读） |
| `outline-xml` | 大纲 XML 格式 |

**Query Hints (Elastic Search Filters):**

| Hint | Description | Example |
|------|-------------|---------|
| `ext:<extension>` | 按扩展名过滤 | `ext:rs` `ext:py` |
| `file:<pattern>` | 按文件路径过滤 | `file:src/**/*.rs` |
| `dir:<pattern>` | 按目录过滤 | `dir:tests` `dir:lib` |
| `type:<filetype>` | 按 ripgrep 类型过滤 | `type:rust` `type:py` |
| `lang:<language>` | 按编程语言过滤 | `lang:javascript` |

**Query Syntax:**
- **AND**: `pattern1 AND pattern2`（必须同时包含）
- **OR**: `pattern1 OR pattern2`（包含任一）
- **NOT**: 暂不支持
- **Phrase**: `"exact phrase"`（精确短语）
- **Default**: 多个词默认为 AND

**Output (outline):**
```
src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
42: pub fn handle_request(req: Request) -> Result<Response> {
43:     // Process the incoming request
44:     let parsed = parse_request(req);
45:     handle(parsed)
46: }
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Score: 0.85
Matched: "handler", "request"

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Output (json):**
```json
{
  "results": [
    {
      "file": "src/main.rs",
      "line": 42,
      "end_line": 46,
      "code": "pub fn handle_request(req: Request) -> Result<Response> { ... }",
      "score": 0.85,
      "matched_terms": ["handler", "request"]
    }
  ],
  "total_results": 1,
  "search_time_ms": 125,
  "query_plan": {
    "terms": ["handler", "request"],
    "filters": ["ext:rs"]
  }
}
```

**Examples:**
```bash
# 基础搜索
probe search "handler" .
probe search "handler" src/

# 多模式搜索（默认 AND）
probe search "handler AND request" src/

# 使用过滤器
probe search "handler ext:rs" .
probe search "handler type:rust" src/
probe search "auth AND file:src/**/*.py" .

# 精确搜索（禁用分词）
probe search "handleRequest" --exact

# 限制结果
probe search "handler" --max-results 5
probe search "handler" --max-tokens 8192

# 禁用代码块合并
probe search "handler" --no-merge
probe search "handler" --merge-threshold 20

# 会话缓存（分页）
probe search "handler" --session search-001
probe search "handler" --session search-001 --max-results 10

# 详细输出
probe search "handler" -v
probe search "handler" --verbose

# JSON 输出
probe search "handler" --format json
probe search "handler" -o json

# 仅文件名
probe search "handler" --files-only

# 排除文件名匹配
probe search "handler" --exclude-filenames

# 自定义忽略模式
probe search "handler" --ignore "**/node_modules" --ignore "**/*.min.js"
```

**Use Cases:**
1. **查找特定函数**: `probe search "handle_request"`
2. **跨文件搜索**: `probe search "auth AND user"`（必须同时包含）
3. **语言过滤**: `probe search "parse ext:rs"`（仅 Rust）
4. **分页检索**: `probe search "handler" --session search-001 --max-results 10`
5. **AI 上下文**: `probe search "auth" --max-tokens 8192 --format json`
6. **代码审查**: `probe search "unsafe" --format outline -v`
