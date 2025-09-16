# JetMD - 高速Markdownプロセッサー

[![Build Status](https://img.shields.io/badge/build-passing-green)](https://github.com)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://github.com)

## 特徴

JetMDは**高性能**な*Markdownプロセッサー*です。

### パフォーマンス

- ⚡ **100 MB/s**の処理速度
- 🎯 **<3ms**のレイテンシー（50KBドキュメント）
- 💾 メモリ使用量は入力の**1.5倍**程度

## インストール

```bash
npm install faster-md
# または
pnpm add faster-md
```

## 使用方法

```javascript
import { renderHtml } from 'faster-md';

const html = await renderHtml('# Hello World');
console.log(html);
```

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

MIT © 2024