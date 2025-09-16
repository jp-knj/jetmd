// Test GFM features specifically
import { renderHtml, parse } from './packages/faster-md/dist/index.js';

async function test() {
  console.log('Testing GFM features...\n');
  
  // Test strikethrough
  console.log('1. Strikethrough test:');
  const strike1 = await renderHtml('~~strikethrough~~', { gfm: true });
  console.log('   Input: ~~strikethrough~~');
  console.log('   Output:', strike1);
  console.log('   Expected: <p><del>strikethrough</del></p>');
  console.log('   Pass:', strike1.includes('<del>') ? '✅' : '❌');
  
  // Test table
  console.log('\n2. Table test:');
  const table1 = await renderHtml('| A | B |\n|---|---|\n| 1 | 2 |', { gfm: true });
  console.log('   Input: | A | B |...');
  console.log('   Output:', table1);
  console.log('   Pass:', table1.includes('<table>') ? '✅' : '❌');
  
  // Test task list
  console.log('\n3. Task list test:');
  const task1 = await renderHtml('- [x] Done\n- [ ] Todo', { gfm: true });
  console.log('   Input: - [x] Done...');
  console.log('   Output:', task1);
  console.log('   Pass:', task1.includes('checkbox') ? '✅' : '❌');
  
  // Parse and check AST
  console.log('\n4. AST test for GFM:');
  const ast = await parse('~~strike~~ and | table |', { gfm: true });
  console.log('   AST:', JSON.stringify(ast, null, 2));
}

test().catch(console.error);