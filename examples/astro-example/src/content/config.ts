import { z, defineCollection } from 'astro:content';

const blogCollection = defineCollection({
  type: 'content', // Markdownコンテンツ
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.date(),
    author: z.string(),
    tags: z.array(z.string()).optional(),
    draft: z.boolean().default(false),
  }),
});

export const collections = {
  'blog': blogCollection,
};