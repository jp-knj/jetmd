// Comprehensive test to verify everything is working
import { parse, renderHtml } from './packages/faster-md/dist/index.js'

async function runTests() {
  let passed = 0
  let failed = 0

  console.log('='.repeat(60))
  console.log('COMPREHENSIVE FUNCTIONALITY TEST')
  console.log('='.repeat(60))

  // Test cases
  const tests = [
    {
      name: 'Basic paragraph',
      input: 'Hello world',
      expectedHtml: '<p>Hello world</p>\n',
      checkAst: (ast) => ast.ast.children[0].type === 'paragraph',
    },
    {
      name: 'Heading',
      input: '# Title\n## Subtitle',
      expectedHtml: '<h1>Title</h1>\n<h2>Subtitle</h2>\n',
      checkAst: (ast) => ast.ast.children[0].type === 'heading' && ast.ast.children[0].depth === 1,
    },
    {
      name: 'Bold and italic',
      input: '**bold** and *italic*',
      expectedHtml: '<p><strong>bold</strong> and <em>italic</em></p>\n',
      checkAst: (ast) => {
        const p = ast.ast.children[0]
        return (
          p.children.some((c) => c.type === 'strong') &&
          p.children.some((c) => c.type === 'emphasis')
        )
      },
    },
    {
      name: 'Code block',
      input: '```js\nconst x = 1;\n```',
      expectedHtml: '<pre><code class="language-js">const x = 1;\n</code></pre>\n',
      checkAst: (ast) => ast.ast.children[0].type === 'code' && ast.ast.children[0].lang === 'js',
    },
    {
      name: 'Inline code',
      input: 'Use `code` here',
      expectedHtml: '<p>Use <code>code</code> here</p>\n',
      checkAst: (ast) => ast.ast.children[0].children.some((c) => c.type === 'inlineCode'),
    },
    {
      name: 'Link',
      input: '[GitHub](https://github.com)',
      expectedHtml: '<p><a href="https://github.com">GitHub</a></p>\n',
      checkAst: (ast) => {
        const link = ast.ast.children[0].children[0]
        return link.type === 'link' && link.url === 'https://github.com'
      },
    },
    {
      name: 'Unordered list',
      input: '- Item 1\n- Item 2\n- Item 3',
      expectedHtml: '<ul>\n<li>Item 1</li>\n<li>Item 2</li>\n<li>Item 3</li>\n</ul>\n',
      checkAst: (ast) => {
        const list = ast.ast.children[0]
        return list.type === 'list' && !list.ordered && list.children.length === 3
      },
    },
    {
      name: 'Ordered list',
      input: '1. First\n2. Second\n3. Third',
      expectedHtml: '<ol>\n<li>First</li>\n<li>Second</li>\n<li>Third</li>\n</ol>\n',
      checkAst: (ast) => {
        const list = ast.ast.children[0]
        return list.type === 'list' && list.ordered && list.children.length === 3
      },
    },
    {
      name: 'Blockquote',
      input: '> This is a quote\n> with multiple lines',
      expectedHtml: '<blockquote>\n<p>This is a quote\nwith multiple lines</p>\n</blockquote>\n',
      checkAst: (ast) => ast.ast.children[0].type === 'blockquote',
    },
    {
      name: 'Horizontal rule',
      input: 'Text\n\n---\n\nMore text',
      expectedHtml: '<p>Text</p>\n<hr />\n<p>More text</p>\n',
      checkAst: (ast) => ast.ast.children[1].type === 'thematicBreak',
    },
    {
      name: 'GFM: Strikethrough',
      input: '~~strikethrough~~',
      expectedHtml: '<p><del>strikethrough</del></p>\n',
      options: { gfm: true },
      checkAst: (ast) => ast.ast.children[0].children[0].type === 'delete',
    },
    {
      name: 'GFM: Table',
      input: '| A | B |\n|---|---|\n| 1 | 2 |',
      expectedHtml:
        '<table>\n<thead>\n<tr><td>A</td><td>B</td></tr>\n</thead>\n<tbody>\n<tr><td>1</td><td>2</td></tr>\n</tbody>\n</table>\n',
      options: { gfm: true },
      checkAst: (ast) => ast.ast.children[0].type === 'table',
    },
    {
      name: 'GFM: Task list',
      input: '- [x] Completed\n- [ ] Not completed',
      expectedHtml:
        '<ul>\n<li><input type="checkbox" checked disabled /> Completed</li>\n<li><input type="checkbox" disabled /> Not completed</li>\n</ul>\n',
      options: { gfm: true },
      checkAst: (ast) => {
        const items = ast.ast.children[0].children
        return items[0].checked === true && items[1].checked === false
      },
    },
    {
      name: 'Nested structures',
      input: '> ## Quote heading\n> \n> With **bold** text',
      expectedHtml:
        '<blockquote>\n<h2>Quote heading</h2>\n<p>With <strong>bold</strong> text</p>\n</blockquote>\n',
      checkAst: (ast) => {
        const bq = ast.ast.children[0]
        return bq.type === 'blockquote' && bq.children[0].type === 'heading'
      },
    },
    {
      name: 'Complex document',
      input: `# Main Title

This is a paragraph with **bold**, *italic*, and \`code\`.

## Features

- Fast parsing
- GFM support
- WASM powered

### Code Example

\`\`\`javascript
function hello() {
  return "world";
}
\`\`\`

> **Note:** This is important!

[Learn more](https://example.com)`,
      checkAst: (ast) => {
        return ast.ast.children.length >= 6 && ast.success === true
      },
    },
  ]

  // Run tests
  for (const test of tests) {
    try {
      console.log(`\n📝 Testing: ${test.name}`)

      // Test HTML rendering
      const html = await renderHtml(test.input, test.options || {})
      console.log(`  Input: ${test.input.substring(0, 50)}${test.input.length > 50 ? '...' : ''}`)
      console.log(`  HTML: ${html.substring(0, 100)}${html.length > 100 ? '...' : ''}`)

      // Test AST parsing
      const ast = await parse(test.input, test.options || {})
      console.log(`  AST nodes: ${countNodes(ast.ast)}`)
      console.log(
        `  Parse time: ${ast.parseTime ? (ast.parseTime / 1000000).toFixed(2) + 'ms' : 'N/A'}`,
      )

      // Check AST structure if provided
      if (test.checkAst) {
        if (test.checkAst(ast)) {
          console.log(`  ✅ AST structure correct`)
        } else {
          console.log(`  ❌ AST structure incorrect`)
          console.log(`  AST:`, JSON.stringify(ast.ast, null, 2).substring(0, 200))
          failed++
          continue
        }
      }

      // Check expected HTML if provided
      if (test.expectedHtml) {
        if (html === test.expectedHtml) {
          console.log(`  ✅ HTML matches expected`)
        } else {
          console.log(`  ❌ HTML mismatch`)
          console.log(`    Expected: ${test.expectedHtml}`)
          console.log(`    Got:      ${html}`)
          failed++
          continue
        }
      }

      passed++
      console.log(`  ✅ PASS`)
    } catch (error) {
      console.log(`  ❌ FAIL: ${error.message}`)
      failed++
    }
  }

  // Performance test
  console.log('\n' + '='.repeat(60))
  console.log('PERFORMANCE TEST')
  console.log('='.repeat(60))

  const perfDoc = '# Title\n\n' + 'This is a paragraph. '.repeat(1000)
  console.log(`Document size: ${perfDoc.length} bytes`)

  // Warm up
  for (let i = 0; i < 10; i++) {
    await renderHtml(perfDoc)
  }

  // Benchmark
  const iterations = 100
  const start = Date.now()
  for (let i = 0; i < iterations; i++) {
    await renderHtml(perfDoc)
  }
  const elapsed = Date.now() - start

  console.log(`Iterations: ${iterations}`)
  console.log(`Total time: ${elapsed}ms`)
  console.log(`Average: ${(elapsed / iterations).toFixed(2)}ms per render`)
  console.log(
    `Throughput: ${((perfDoc.length * iterations) / (elapsed / 1000) / 1024 / 1024).toFixed(
      2,
    )} MB/s`,
  )

  // Summary
  console.log('\n' + '='.repeat(60))
  console.log('SUMMARY')
  console.log('='.repeat(60))
  console.log(`✅ Passed: ${passed}/${tests.length}`)
  console.log(`❌ Failed: ${failed}/${tests.length}`)

  if (failed === 0) {
    console.log('\n🎉 All tests passed! The implementation is working correctly.')
  } else {
    console.log('\n⚠️  Some tests failed. Please review the errors above.')
  }

  process.exit(failed === 0 ? 0 : 1)
}

function countNodes(node) {
  if (!node) return 0
  let count = 1
  if (node.children) {
    for (const child of node.children) {
      count += countNodes(child)
    }
  }
  return count
}

runTests().catch((error) => {
  console.error('Fatal error:', error)
  process.exit(1)
})
