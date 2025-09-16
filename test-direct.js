// Test rendering directly from AST
import { renderHtml, parse } from './packages/faster-md/dist/index.js';

async function test() {
  // Parse with GFM
  const ast = await parse('~~strikethrough~~', { gfm: true });
  console.log('Parsed AST:');
  console.log(JSON.stringify(ast, null, 2));
  
  // Check if strikethrough was parsed
  const hasDelete = JSON.stringify(ast).includes('"delete"');
  console.log('\nHas delete node:', hasDelete ? '✅' : '❌');
  
  // Try to render the AST
  try {
    const html = await renderHtml(ast.ast, { gfm: true });
    console.log('\nRendered from AST:', html);
  } catch (e) {
    console.log('\nCannot render AST directly:', e.message);
  }
  
  // Direct render
  const html2 = await renderHtml('~~strikethrough~~', { gfm: true });
  console.log('\nDirect render:', html2);
  
  // Check if the issue is in parsing or rendering
  console.log('\n--- Diagnosis ---');
  console.log('Parsing works:', hasDelete ? '✅' : '❌');
  console.log('Rendering works:', html2.includes('<del>') ? '✅' : '❌');
}

test().catch(console.error);