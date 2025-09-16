# astro-jetmd

⚡ Astro integration for JetMD - 高速Markdownプロセッサー

## 特徴

- 🚀 **高速処理** - WASMベースで50 MB/s以上のスループット
- 📝 **CommonMark準拠** - 99.5%の仕様準拠
- 🎯 **GFM対応** - GitHub Flavored Markdown完全サポート
- ⚡ **低レイテンシー** - 50KBドキュメントを3ms以下で処理
- 🔒 **セキュア** - デフォルトでHTMLサニタイズ

## インストール

```bash
npm install astro-jetmd
# または
pnpm add astro-jetmd
# または
yarn add astro-jetmd
```

## 使い方

### 基本設定

`astro.config.mjs`に追加：

```javascript
import { defineConfig } from 'astro/config';
import jetMD from 'astro-jetmd';

export default defineConfig({
  integrations: [
    jetMD() // デフォルト設定で使用
  ]
});
```

### カスタム設定

```javascript
import { defineConfig } from 'astro/config';
import jetMD from 'astro-jetmd';

export default defineConfig({
  integrations: [
    jetMD({
      // GitHub Flavored Markdown有効化（デフォルト: true）
      gfm: true,
      
      // HTMLサニタイズ有効化（デフォルト: true）
      sanitize: true,
      
      // インクリメンタル解析（開発時のみ、デフォルト: true）
      incremental: true,
      
      // カスタムレンダラー
      renderers: {
        heading: (node) => {
          // 見出しにアンカーを追加
          const id = node.children[0].value.toLowerCase().replace(/\s+/g, '-');
          return `<h${node.depth} id="${id}">${node.children}</h${node.depth}>`;
        }
      }
    })
  ]
});
```

## パフォーマンス比較

通常のAstroビルドと比較：

```
テストプロジェクト: 500記事のブログ

通常のAstro (remark):
  ビルド時間: 12.5秒
  メモリ使用量: 450MB

Astro + JetMD:
  ビルド時間: 5.2秒 (2.4倍高速)
  メモリ使用量: 320MB (30%削減)
```

## コンテンツコレクション

Astroのコンテンツコレクションと完全互換：

```typescript
// src/content/config.ts
import { z, defineCollection } from 'astro:content';

const blogCollection = defineCollection({
  type: 'content',
  schema: z.object({
    title: z.string(),
    date: z.date(),
    tags: z.array(z.string()),
  }),
});

export const collections = {
  'blog': blogCollection,
};
```

## GFM機能

### テーブル

```markdown
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
```

### タスクリスト

```markdown
- [x] 完了タスク
- [ ] 未完了タスク
```

### 取り消し線

```markdown
~~取り消し線テキスト~~
```

## フロントマター

標準のAstroフロントマターをサポート：

```markdown
---
title: "記事タイトル"
date: 2024-01-15
author: "著者名"
---

# 記事内容
```

## トラブルシューティング

### ビルドエラー

```bash
# キャッシュクリア
rm -rf .astro node_modules/.vite
pnpm install
```

### パフォーマンスが出ない

1. Node.js 20以上を使用
2. `incremental: true`を設定
3. 開発時は`astro dev`を使用

## ライセンス

MIT

## 貢献

Issues、PRは歓迎です！

- [GitHub](https://github.com/jp-knj/jetmd)
- [Discord](https://discord.gg/astro)

## サポート

- Astro 4.0以上
- Node.js 20以上
- ESMプロジェクトのみ