// Incremental parsing example for real-time editors
import { clearSession, createParseSession, parseIncremental } from 'faster-md'

// Simulate a real-time editor with incremental updates
async function simulateEditor() {
  // Create a new parsing session
  const sessionId = await createParseSession('editor-session-1')
  console.log('Created session:', sessionId)

  // Initial document
  let document = '# My Document\n\nInitial content.'
  let ast = await parseIncremental(sessionId, document)
  console.log('Initial AST nodes:', countNodes(ast))

  // Simulate user typing - adding a paragraph
  document += '\n\nThis is a new paragraph.'
  const edit1 = {
    type: 'insert',
    offset: document.length - 25,
    text: '\n\nThis is a new paragraph.',
  }

  console.time('Incremental parse 1')
  ast = await parseIncremental(sessionId, document, [edit1])
  console.timeEnd('Incremental parse 1')
  console.log('After edit 1 - nodes:', countNodes(ast))

  // Simulate user editing - modifying the title
  document = document.replace('# My Document', '# Updated Document')
  const edit2 = {
    type: 'replace',
    offset: 0,
    length: 13,
    text: '# Updated Document',
  }

  console.time('Incremental parse 2')
  ast = await parseIncremental(sessionId, document, [edit2])
  console.timeEnd('Incremental parse 2')
  console.log('After edit 2 - nodes:', countNodes(ast))

  // Simulate adding a list
  document += '\n\n- Item 1\n- Item 2\n- Item 3'
  const edit3 = {
    type: 'insert',
    offset: document.length - 26,
    text: '\n\n- Item 1\n- Item 2\n- Item 3',
  }

  console.time('Incremental parse 3')
  ast = await parseIncremental(sessionId, document, [edit3])
  console.timeEnd('Incremental parse 3')
  console.log('After edit 3 - nodes:', countNodes(ast))

  // Clear the session when done
  await clearSession(sessionId)
  console.log('Session cleared')

  return ast
}

// Helper function to count AST nodes
function countNodes(node) {
  let count = 1
  if (node.children && Array.isArray(node.children)) {
    for (const child of node.children) {
      count += countNodes(child)
    }
  }
  return count
}

// Example: Multiple concurrent sessions
async function multipleSessions() {
  // Create multiple sessions for different editors/tabs
  const sessions = await Promise.all([
    createParseSession('tab-1'),
    createParseSession('tab-2'),
    createParseSession('tab-3'),
  ])

  console.log('Created sessions:', sessions)

  // Parse different documents in parallel
  const results = await Promise.all([
    parseIncremental(sessions[0], '# Document 1'),
    parseIncremental(sessions[1], '# Document 2'),
    parseIncremental(sessions[2], '# Document 3'),
  ])

  console.log(
    'Parsed documents:',
    results.map((ast) => ast.children?.[0]?.children?.[0]?.value),
  )

  // Clean up
  await Promise.all(sessions.map((sid) => clearSession(sid)))
}

// Example: Caching and performance
async function cachingDemo() {
  const sessionId = await createParseSession('cache-demo')
  const document = '# Title\n\n' + 'Paragraph. '.repeat(100)

  // First parse - cold cache
  console.time('Cold parse')
  let ast = await parseIncremental(sessionId, document)
  console.timeEnd('Cold parse')

  // Second parse - same content, should use cache
  console.time('Cached parse')
  ast = await parseIncremental(sessionId, document)
  console.timeEnd('Cached parse')

  // Small edit
  const editedDoc = document + '\n\nNew line.'
  console.time('Incremental parse')
  ast = await parseIncremental(sessionId, editedDoc, [
    {
      type: 'insert',
      offset: document.length,
      text: '\n\nNew line.',
    },
  ])
  console.timeEnd('Incremental parse')

  await clearSession(sessionId)
}

// Run examples
async function main() {
  console.log('=== Simulated Editor ===')
  const finalAst = await simulateEditor()

  console.log('\n=== Multiple Sessions ===')
  await multipleSessions()

  console.log('\n=== Caching Demo ===')
  await cachingDemo()

  console.log('\n=== Final AST Structure ===')
  console.log(JSON.stringify(finalAst, null, 2))
}

main().catch(console.error)
