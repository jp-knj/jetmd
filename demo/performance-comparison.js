#!/usr/bin/env node

// パフォーマンス比較テスト
import { performance } from 'node:perf_hooks';
import { renderHtml } from '../packages/faster-md/dist/index.js';

async function detailedBenchmark() {
  console.log('🏃 JetMD パフォーマンス詳細分析\n');
  console.log('='.repeat(60));

  // テストドキュメントサイズ別
  const testCases = [
    { name: '小 (1KB)', size: 1 },
    { name: '中 (10KB)', size: 10 },
    { name: '大 (50KB)', size: 50 },
    { name: '特大 (100KB)', size: 100 },
    { name: '巨大 (500KB)', size: 500 }
  ];

  const results = [];

  for (const testCase of testCases) {
    // ドキュメント生成
    const doc = generateDocument(testCase.size);
    const actualSize = (doc.length / 1024).toFixed(2);
    
    console.log(`\n📄 ${testCase.name} - ${actualSize}KB`);
    console.log('-'.repeat(40));

    // ウォームアップ
    for (let i = 0; i < 5; i++) {
      await renderHtml(doc, { gfm: true });
    }

    // ベンチマーク
    const iterations = testCase.size > 100 ? 20 : 100;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
      const start = performance.now();
      await renderHtml(doc, { gfm: true });
      const elapsed = performance.now() - start;
      times.push(elapsed);
    }

    // 統計計算
    times.sort((a, b) => a - b);
    const min = times[0];
    const max = times[times.length - 1];
    const median = times[Math.floor(times.length / 2)];
    const avg = times.reduce((a, b) => a + b, 0) / times.length;
    const p95 = times[Math.floor(times.length * 0.95)];
    const p99 = times[Math.floor(times.length * 0.99)];

    // スループット計算 (MB/s)
    const throughput = (doc.length / (avg / 1000)) / (1024 * 1024);

    console.log(`  最小: ${min.toFixed(3)}ms`);
    console.log(`  中央値: ${median.toFixed(3)}ms`);
    console.log(`  平均: ${avg.toFixed(3)}ms`);
    console.log(`  P95: ${p95.toFixed(3)}ms`);
    console.log(`  P99: ${p99.toFixed(3)}ms`);
    console.log(`  最大: ${max.toFixed(3)}ms`);
    console.log(`  ⚡ スループット: ${throughput.toFixed(2)} MB/s`);

    results.push({
      size: testCase.name,
      sizeKB: parseFloat(actualSize),
      avg,
      median,
      p95,
      p99,
      throughput
    });
  }

  // 比較表
  console.log('\n' + '='.repeat(60));
  console.log('📊 パフォーマンスサマリー');
  console.log('='.repeat(60));
  
  console.log('\n| サイズ | 中央値 | P95 | P99 | スループット |');
  console.log('|--------|--------|-----|-----|-------------|');
  for (const r of results) {
    console.log(
      `| ${r.size} | ${r.median.toFixed(2)}ms | ${r.p95.toFixed(2)}ms | ${r.p99.toFixed(2)}ms | ${r.throughput.toFixed(1)} MB/s |`
    );
  }

  // 目標との比較
  console.log('\n' + '='.repeat(60));
  console.log('🎯 目標達成状況');
  console.log('='.repeat(60));
  
  const target50KB = results.find(r => r.size === '大 (50KB)');
  if (target50KB) {
    console.log('\n50KBドキュメント:');
    console.log(`  目標: <3ms (P50)、<15ms (P99)`);
    console.log(`  実績: ${target50KB.median.toFixed(2)}ms (P50)、${target50KB.p99.toFixed(2)}ms (P99)`);
    
    const p50Pass = target50KB.median < 3;
    const p99Pass = target50KB.p99 < 15;
    console.log(`  P50: ${p50Pass ? '✅ PASS' : '❌ FAIL'}`);
    console.log(`  P99: ${p99Pass ? '✅ PASS' : '❌ FAIL'}`);
  }

  const avgThroughput = results.reduce((sum, r) => sum + r.throughput, 0) / results.length;
  console.log(`\n平均スループット: ${avgThroughput.toFixed(2)} MB/s`);
  console.log(`目標: ≥50 MB/s`);
  console.log(avgThroughput >= 50 ? '✅ 目標達成！' : '⚠️  要改善');

  // 他のライブラリとの比較（参考値）
  console.log('\n' + '='.repeat(60));
  console.log('📈 他のMarkdownパーサーとの比較（参考値）');
  console.log('='.repeat(60));
  console.log(`
| ライブラリ | スループット | 相対速度 |
|------------|-------------|----------|
| JetMD (我々) | ${avgThroughput.toFixed(1)} MB/s | 1.0x |
| marked | ~30-40 MB/s | ${(avgThroughput / 35).toFixed(1)}x faster |
| markdown-it | ~20-30 MB/s | ${(avgThroughput / 25).toFixed(1)}x faster |
| remark | ~10-20 MB/s | ${(avgThroughput / 15).toFixed(1)}x faster |
| showdown | ~5-10 MB/s | ${(avgThroughput / 7.5).toFixed(1)}x faster |

※ 参考値は一般的なベンチマーク結果に基づく推定値
`);

  return results;
}

// ドキュメント生成
function generateDocument(sizeKB) {
  const chunks = [];
  let currentSize = 0;
  const targetSize = sizeKB * 1024;

  // ヘッダー
  chunks.push('# Benchmark Document\n\n');
  currentSize += chunks[0].length;

  // パラグラフ
  const paragraph = 'This is a test paragraph with **bold**, *italic*, and `inline code`. ';
  while (currentSize < targetSize * 0.4) {
    chunks.push(paragraph);
    currentSize += paragraph.length;
  }
  chunks.push('\n\n');

  // リスト
  chunks.push('## List Section\n\n');
  const listItem = '- List item with some text\n';
  while (currentSize < targetSize * 0.5) {
    chunks.push(listItem);
    currentSize += listItem.length;
  }
  chunks.push('\n');

  // コードブロック
  chunks.push('## Code Section\n\n```javascript\n');
  const codeLine = 'const value = Math.random() * 100;\n';
  while (currentSize < targetSize * 0.7) {
    chunks.push(codeLine);
    currentSize += codeLine.length;
  }
  chunks.push('```\n\n');

  // テーブル (GFM)
  chunks.push('## Table Section\n\n');
  chunks.push('| Column A | Column B | Column C |\n');
  chunks.push('|----------|----------|----------|\n');
  const tableRow = '| Data | Data | Data |\n';
  while (currentSize < targetSize * 0.85) {
    chunks.push(tableRow);
    currentSize += tableRow.length;
  }
  chunks.push('\n');

  // 引用
  chunks.push('> This is a blockquote with **formatted** text.\n\n');

  // パディング
  while (currentSize < targetSize) {
    chunks.push('Extra text. ');
    currentSize += 12;
  }

  return chunks.join('');
}

// 実行
detailedBenchmark().catch(console.error);