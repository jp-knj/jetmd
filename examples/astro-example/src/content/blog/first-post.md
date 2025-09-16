---
title: "JetMDでAstroを高速化"
description: "WASMベースの高速Markdownプロセッサーを使ってAstroサイトを高速化する方法"
date: 2024-01-15
author: "Astro Developer"
tags: ["astro", "performance", "markdown"]
---

# JetMDでAstroを高速化

Astroサイトの**ビルド時間**を大幅に短縮する方法をご紹介します。

## なぜJetMD？

従来のMarkdownプロセッサーと比較して：

| プロセッサー | スループット | 相対速度 |
|-------------|-------------|----------|
| JetMD | 50 MB/s | ⚡ 1.0x |
| marked | 30 MB/s | 1.6x faster |
| markdown-it | 20 MB/s | 2.5x faster |
| remark | 15 MB/s | 3.3x faster |

## 主な特徴

- ✅ **CommonMark準拠** - 99.5%の仕様準拠
- ✅ **GFM対応** - GitHub Flavored Markdown完全サポート
- ✅ **高速** - WASMによる最適化
- ✅ **安全** - デフォルトでHTMLサニタイズ

## インストール方法

```bash
npm install astro-jetmd
```

## 設定

`astro.config.mjs`に追加：

```javascript
import { defineConfig } from 'astro/config';
import jetMD from 'astro-jetmd';

export default defineConfig({
  integrations: [
    jetMD({
      gfm: true,
      sanitize: true
    })
  ]
});
```

## パフォーマンステスト結果

実際のプロジェクトでのテスト結果：

- 📄 **1000記事**: 従来15秒 → JetMDで6秒（2.5倍高速）
- 📦 **ビルドサイズ**: 変化なし
- 💾 **メモリ使用量**: 約30%削減

## コードサンプル

```javascript
// 高度な使用例
import { renderHtml } from 'faster-md';

const html = await renderHtml(markdown, {
  gfm: true,
  sanitize: true,
  renderers: {
    heading: (node) => {
      // カスタムレンダリング
      return `<h${node.depth} class="custom">${node.children}</h${node.depth}>`;
    }
  }
});
```

## タスクリスト例

- [x] JetMDインストール
- [x] Astro設定更新
- [x] パフォーマンス測定
- [ ] プロダクションデプロイ

## まとめ

JetMDを使うことで、Astroサイトのビルド時間を**大幅に短縮**できます。
特に記事数が多いブログサイトでは効果的です。

> **Note**: 現在MDXサポートは開発中です。

---

*この記事はJetMDで処理されています ⚡*