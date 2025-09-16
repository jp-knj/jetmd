import { defineConfig } from 'astro/config';
import jetMD from 'astro-jetmd';

// https://astro.build/config
export default defineConfig({
  integrations: [
    // JetMDを使用して高速Markdown処理
    jetMD({
      gfm: true,        // GitHub Flavored Markdown有効
      sanitize: true,   // HTMLサニタイズ有効
      incremental: true // 開発時のインクリメンタル解析
    })
  ],
  
  // その他のAstro設定
  site: 'https://example.com',
  output: 'static'
});