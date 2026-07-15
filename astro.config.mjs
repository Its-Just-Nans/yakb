// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { unified } from "@astrojs/markdown-remark";
import rehypeKatex from "rehype-katex";
import rehypeExternalLinks from "rehype-external-links";
import remarkMath from "remark-math";
import remarkToc from "remark-toc";
import starlightLinksValidator from "starlight-links-validator";

// https://astro.build/config
export default defineConfig({
  site: "https://yakb.n4n5.dev",
  markdown: {
    processor: unified({
      remarkPlugins: [remarkMath, remarkToc],
      rehypePlugins: [rehypeKatex, [rehypeExternalLinks, { target: "_blank" }]],
    }),
  },
  integrations: [
    starlight({
      plugins: [starlightLinksValidator()],
      title: "YakB",
      favicon: "/favicon.ico",
      logo: {
        light: "./src/assets/logo/yakb.svg",
        dark: "./src/assets/logo/yakb.svg",
      },
      editLink: {
        baseUrl: "https://github.com/Its-Just-Nans/yakb/edit/main/",
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/Its-Just-Nans/yakb",
        },
      ],
      sidebar: [
        {
          autogenerate: { directory: "guides", collapsed: true },
        },
      ],
      customCss: ["./src/styles/custom.css"],
    }),
  ],
  image: {
    // Example: Enable the Sharp-based image service with a custom config
    service: {
      entrypoint: "astro/assets/services/sharp",
      config: {
        limitInputPixels: false,
      },
    },
  },
});
