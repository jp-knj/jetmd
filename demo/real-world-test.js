#!/usr/bin/env node

// 実践的なテスト - Real-world usage test
import { readFile, readdir, writeFile } from 'node:fs/promises'
import { basename, join } from 'node:path'
import { performance } from 'node:perf_hooks'
import { renderHtml } from '../packages/faster-md/dist/index.js'

// 実際のMarkdownファイルを処理してHTMLに変換
async function convertMarkdownFile(filePath) {
  console.log(`\n📄 Processing: ${basename(filePath)}`)

  try {
    const markdown = await readFile(filePath, 'utf-8')
    const fileSize = (markdown.length / 1024).toFixed(2)
    console.log(`   Size: ${fileSize} KB`)

    // パフォーマンス測定
    const start = performance.now()
    const html = await renderHtml(markdown, {
      gfm: true,
      sanitize: true,
    })
    const elapsed = performance.now() - start

    // 結果を保存
    const outputPath = filePath.replace(/\.md$/i, '.html')
    await writeFile(outputPath, wrapHtml(html, basename(filePath)))

    console.log(`   ✅ Converted in ${elapsed.toFixed(2)}ms`)
    console.log(`   📁 Output: ${basename(outputPath)}`)

    return { success: true, elapsed, size: markdown.length }
  } catch (error) {
    console.log(`   ❌ Error: ${error.message}`)
    return { success: false, error: error.message }
  }
}

// HTMLラッパー
function wrapHtml(content, title) {
  return `<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>${title}</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
      line-height: 1.6;
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem;
      color: #333;
    }
    pre {
      background: #f4f4f4;
      padding: 1rem;
      border-radius: 4px;
      overflow-x: auto;
    }
    code {
      background: #f4f4f4;
      padding: 0.2rem 0.4rem;
      border-radius: 3px;
    }
    blockquote {
      border-left: 4px solid #ddd;
      margin: 0;
      padding-left: 1rem;
      color: #666;
    }
    table {
      border-collapse: collapse;
      width: 100%;
      margin: 1rem 0;
    }
    th, td {
      border: 1px solid #ddd;
      padding: 0.5rem;
      text-align: left;
    }
    th {
      background: #f4f4f4;
    }
    input[type="checkbox"] {
      margin-right: 0.5rem;
    }
    del {
      text-decoration: line-through;
      color: #999;
    }
  </style>
</head>
<body>
${content}
</body>
</html>`
}

// サンプルMarkdownファイルを作成
async function createSampleFiles() {
  console.log('Creating sample markdown files...\n')

  const samples = {
    'README.md': `# JetMD - 高速Markdownプロセッサー

[![Build Status](https://img.shields.io/badge/build-passing-green)](https://github.com)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com)

## 特徴

JetMDは**高性能**な*Markdownプロセッサー*です。

### パフォーマンス

- ⚡ **100 MB/s**の処理速度
- 🎯 **<3ms**のレイテンシー（50KBドキュメント）
- 💾 メモリ使用量は入力の**1.5倍**程度

## インストール

\`\`\`bash
npm install faster-md
# または
pnpm add faster-md
\`\`\`

## 使用方法

\`\`\`javascript
import { renderHtml } from 'faster-md';

const html = await renderHtml('# Hello World');
console.log(html);
\`\`\`

## GFM機能

### テーブル

| 機能 | サポート | パフォーマンス |
|------|----------|---------------|
| CommonMark | ✅ | 100 MB/s |
| GFM | ✅ | 95 MB/s |
| MDX | 🚧 | TBD |

### タスクリスト

- [x] パーサー実装
- [x] WASM対応
- [x] GFMサポート
- [ ] MDXサポート
- [ ] プラグインシステム

### その他

~~取り消し線~~もサポートしています。

自動リンク: https://github.com/jp-knj/jetmd

## ライセンス

MIT © 2024`,

    'API.md': `# API Documentation

## Core Functions

### \`renderHtml(markdown, options)\`

Markdownを**HTML**に変換します。

#### Parameters

- \`markdown\` (string): 入力Markdown
- \`options\` (object): オプション
  - \`gfm\` (boolean): GFM有効化
  - \`sanitize\` (boolean): サニタイズ

#### Returns

Promise<string> - HTML文字列

#### Example

\`\`\`typescript
const html = await renderHtml(
  '# Title\\n\\nParagraph',
  { gfm: true }
);
\`\`\`

### \`parse(markdown, options)\`

MarkdownをASTに変換します。

> **Note**: このAPIは内部実装に依存します

#### Code Example

\`\`\`javascript
const ast = await parse('# Hello');
console.log(ast);
// Output:
// {
//   type: 'root',
//   children: [{
//     type: 'heading',
//     depth: 1,
//     children: [{
//       type: 'text',
//       value: 'Hello'
//     }]
//   }]
// }
\`\`\`

## Advanced Usage

### Custom Renderers

\`\`\`javascript
import { renderWithCustom } from 'faster-md';

const html = await renderWithCustom(markdown, {
  heading: (node) => {
    const level = node.depth;
    const text = getTextContent(node);
    return \`<h\${level} id="\${slugify(text)}">\${text}</h\${level}>\`;
  }
});
\`\`\`

---

[Back to README](./README.md) | [GitHub](https://github.com)`,

    'CHANGELOG.md': `# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-01-15

### Added
- ✨ Initial release
- 🚀 WASM-based parser using pulldown-cmark
- 📝 CommonMark compliance (99.5%)
- 🎯 GFM extensions support
  - Tables
  - Strikethrough
  - Task lists
  - Autolinks
- ⚡ Performance optimizations
  - 100 MB/s throughput
  - <3ms latency for 50KB documents
- 📚 Comprehensive documentation
- 🧪 Test suite with 95% coverage

### Performance Benchmarks

\`\`\`
Document Size | Throughput | Latency (p50) | Memory
-------------|------------|---------------|--------
1 KB         | 120 MB/s   | 0.08ms        | 1.5 KB
10 KB        | 110 MB/s   | 0.9ms         | 15 KB
50 KB        | 100 MB/s   | 1.2ms         | 75 KB
100 KB       | 95 MB/s    | 2.5ms         | 150 KB
\`\`\`

### Fixed
- 🐛 WASM panic with time API
- 🔧 Serde serialization issues
- 🎨 Options passing (object vs JSON string)

### Security
- 🔒 HTML sanitization enabled by default
- 🛡️ XSS protection

## [Unreleased]

### Planned
- [ ] MDX support
- [ ] Incremental parsing optimization
- [ ] Plugin system
- [ ] Native bindings (Neon)

---

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)`,
  }

  for (const [filename, content] of Object.entries(samples)) {
    const filePath = join('demo', filename)
    await writeFile(filePath, content)
    console.log(`✅ Created: ${filename}`)
  }
}

// 比較テスト
async function comparePerformance() {
  console.log(`\n${'='.repeat(60)}`)
  console.log('PERFORMANCE COMPARISON')
  console.log('='.repeat(60))

  const testDoc = `# Benchmark Document

${'This is a paragraph with **bold** and *italic* text. '.repeat(100)}

## Code Example

\`\`\`javascript
${'const x = 1;\n'.repeat(50)}
\`\`\`

## List

${'- List item\n'.repeat(20)}

## Table

| Column A | Column B | Column C |
|----------|----------|----------|
${'| Data | Data | Data |\n'.repeat(10)}
`

  console.log(`\nDocument size: ${(testDoc.length / 1024).toFixed(2)} KB`)

  // Our implementation
  console.log('\n📦 faster-md (Our implementation):')
  const iterations = 100

  // Warm up
  for (let i = 0; i < 10; i++) {
    await renderHtml(testDoc, { gfm: true })
  }

  const start = performance.now()
  for (let i = 0; i < iterations; i++) {
    await renderHtml(testDoc, { gfm: true })
  }
  const ourTime = performance.now() - start
  const ourAvg = ourTime / iterations
  const ourThroughput = (testDoc.length * iterations) / (ourTime / 1000) / 1024 / 1024

  console.log(`  Average: ${ourAvg.toFixed(2)}ms`)
  console.log(`  Throughput: ${ourThroughput.toFixed(2)} MB/s`)
  console.log(`  Total: ${ourTime.toFixed(2)}ms for ${iterations} iterations`)

  // You could add comparisons with other libraries here
  // For example: marked, markdown-it, remark, etc.
}

// メイン処理
async function main() {
  console.log('🚀 JetMD Real-World Test\n')
  console.log('Testing practical usage scenarios...\n')

  // サンプルファイルを作成
  await createSampleFiles()

  // Markdownファイルを処理
  console.log(`\n${'='.repeat(60)}`)
  console.log('CONVERTING MARKDOWN FILES')
  console.log('='.repeat(60))

  const files = await readdir('demo')
  const mdFiles = files.filter((f) => f.endsWith('.md'))

  const results = []
  for (const file of mdFiles) {
    const result = await convertMarkdownFile(join('demo', file))
    results.push(result)
  }

  // 結果のサマリー
  console.log(`\n${'='.repeat(60)}`)
  console.log('SUMMARY')
  console.log('='.repeat(60))

  const successful = results.filter((r) => r.success)
  const totalTime = successful.reduce((sum, r) => sum + r.elapsed, 0)
  const totalSize = successful.reduce((sum, r) => sum + r.size, 0)

  console.log(`\n✅ Converted: ${successful.length}/${results.length} files`)
  console.log(`📊 Total time: ${totalTime.toFixed(2)}ms`)
  console.log(`📦 Total size: ${(totalSize / 1024).toFixed(2)} KB`)
  console.log(`⚡ Average speed: ${(totalSize / (totalTime / 1000) / 1024 / 1024).toFixed(2)} MB/s`)

  // パフォーマンス比較
  await comparePerformance()

  console.log('\n✨ Test complete! Check the demo/*.html files to see the results.')
  console.log('💡 You can open them in a browser to verify the rendering quality.')
}

// エラーハンドリング
main().catch((error) => {
  console.error('❌ Test failed:', error)
  process.exit(1)
})
