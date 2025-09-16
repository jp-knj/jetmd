import type { AstroIntegration } from 'astro'
import { renderHtml } from 'faster-md'
import type { VitePlugin } from 'vite'

export interface AstroJetMDOptions {
  /**
   * Enable GitHub Flavored Markdown
   * @default true
   */
  gfm?: boolean

  /**
   * Enable HTML sanitization
   * @default true
   */
  sanitize?: boolean

  /**
   * Enable MDX support (future)
   * @default false
   */
  mdx?: boolean

  /**
   * Custom renderer functions
   */
  renderers?: Record<string, (node: unknown) => string>

  /**
   * Enable incremental parsing for dev mode
   * @default true
   */
  incremental?: boolean
}

/**
 * Astro integration for JetMD
 * High-performance Markdown processor
 */
export default function astroJetMD(options: AstroJetMDOptions = {}): AstroIntegration {
  const config: AstroJetMDOptions = {
    gfm: true,
    sanitize: true,
    mdx: false,
    incremental: true,
    ...options,
  }

  return {
    name: 'astro-jetmd',
    hooks: {
      'astro:config:setup': ({ updateConfig }) => {
        console.log('⚡ JetMD: Initializing high-performance Markdown processor')

        // Viteプラグインを追加
        updateConfig({
          vite: {
            plugins: [jetMDVitePlugin(config)],
          },
          markdown: {
            // Astroの内蔵Markdownプロセッサーを無効化
            // JetMDで処理
            render: undefined,
          },
        })
      },

      'astro:config:done': () => {
        console.log('✅ JetMD: Configuration complete')
      },

      'astro:build:start': async () => {
        console.log('🏗️  JetMD: Starting build process')
      },

      'astro:build:done': async ({ pages }) => {
        console.log(`✨ JetMD: Built ${pages.length} pages`)
      },
    },
  }
}

/**
 * Vite plugin for processing Markdown with JetMD
 */
function jetMDVitePlugin(options: AstroJetMDOptions): VitePlugin {
  return {
    name: 'vite-plugin-jetmd',

    async transform(code: string, id: string) {
      // .mdファイルを処理
      if (id.endsWith('.md')) {
        try {
          const start = performance.now()

          // フロントマターを抽出
          const { content, frontmatter } = extractFrontmatter(code)

          // JetMDでHTMLに変換
          const html = await renderHtml(content, {
            gfm: options.gfm,
            sanitize: options.sanitize,
          })

          const elapsed = performance.now() - start

          // Astro用のコンポーネントとして出力
          const component = generateAstroComponent(html, frontmatter)

          // デバッグ情報
          if (process.env.NODE_ENV === 'development') {
            console.log(`  📄 ${id.split('/').pop()}: ${elapsed.toFixed(2)}ms`)
          }

          return {
            code: component,
            map: null,
          }
        } catch (error) {
          console.error(`JetMD Error processing ${id}:`, error)
          throw error
        }
      }

      // .mdxファイルのサポート（将来）
      if (id.endsWith('.mdx') && options.mdx) {
        // MDX処理（未実装）
        console.warn('MDX support is not yet implemented')
      }

      return null
    },
  }
}

/**
 * フロントマターを抽出
 */
function extractFrontmatter(content: string): {
  content: string
  frontmatter: Record<string, unknown>
} {
  const frontmatterRegex = /^---\s*\n([\s\S]*?)\n---\s*\n/
  const match = content.match(frontmatterRegex)

  if (match) {
    const frontmatterContent = match[1]
    const mainContent = content.slice(match[0].length)

    // 簡易的なYAMLパース（実際のプロジェクトではyamlライブラリを使用）
    const frontmatter: Record<string, unknown> = {}
    for (const line of frontmatterContent.split('\n')) {
      const [key, ...valueParts] = line.split(':')
      if (key && valueParts.length) {
        const value = valueParts.join(':').trim()
        frontmatter[key.trim()] = value.replace(/^["']|["']$/g, '')
      }
    }

    return { content: mainContent, frontmatter }
  }

  return { content, frontmatter: {} }
}

/**
 * Astroコンポーネントを生成
 */
function generateAstroComponent(html: string, frontmatter: Record<string, unknown>): string {
  const exportStatements = Object.entries(frontmatter)
    .map(([key, value]) => `export const ${key} = ${JSON.stringify(value)};`)
    .join('\n')

  return `
${exportStatements}

const html = ${JSON.stringify(html)};

export default function MarkdownContent() {
  return html;
}

export function getHeadings() {
  // ヘッディング抽出（必要に応じて実装）
  return [];
}
`
}

// TypeScript型定義のエクスポート
export type { AstroIntegration }
