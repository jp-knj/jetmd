// Quick test to debug WASM issue
import { parse, renderHtml } from './packages/faster-md/dist/index.js'

async function test() {
  try {
    console.log('Testing simple markdown...')

    // Test 1: Very simple text
    const result1 = await renderHtml('Hello world', { gfm: false })
    console.log('Test 1 passed:', result1)

    // Test 2: Simple paragraph
    const result2 = await renderHtml('This is a paragraph.', { gfm: false })
    console.log('Test 2 passed:', result2)

    // Test 3: Heading
    const result3 = await renderHtml('# Hello', { gfm: false })
    console.log('Test 3 passed:', result3)

    // Test parse directly
    console.log('\nTesting parse...')
    const ast = await parse('# Hello world', { gfm: false })
    console.log('Parse result:', JSON.stringify(ast, null, 2))

    // Test with more detailed markdown
    const markdown2 = `# Title
    
This is a paragraph with **bold** and *italic* text.

- List item 1
- List item 2
`
    const ast2 = await parse(markdown2, { gfm: true })
    console.log('Parse result 2:', JSON.stringify(ast2, null, 2))
  } catch (error) {
    console.error('Test failed:', error)
    console.error('Stack:', error.stack)
  }
}

test()
