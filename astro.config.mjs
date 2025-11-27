// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import rehypeKatex from "rehype-katex";
import rehypeExternalLinks from "rehype-external-links";
import remarkMath from "remark-math";
import remarkToc from "remark-toc";
import starlightLinksValidator from "starlight-links-validator";
import { download } from "./src/utils";

await download("public/downloaded", {
    "https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/katex.min.css": "katex.min.css",
    "https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/fonts/KaTeX_Main-Regular.woff2": "fonts/KaTeX_Main-Regular.woff2",
    "https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/fonts/KaTeX_Main-Italic.woff2": "fonts/KaTeX_Main-Italic.woff2",
    "https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/fonts/KaTeX_Math-Italic.woff2": "fonts/KaTeX_Math-Italic.woff2",
    "https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/fonts/KaTeX_Size2-Regular.woff2":
        "fonts/KaTeX_Size2-Regular.woff2",
    "https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/fonts/KaTeX_AMS-Regular.woff2": "fonts/KaTeX_AMS-Regular.woff2",
});

// https://astro.build/config
export default defineConfig({
    site: "https://yakb.n4n5.dev",
    markdown: {
        remarkPlugins: [remarkMath, remarkToc],
        rehypePlugins: [rehypeKatex, [rehypeExternalLinks, { target: "_blank" }]],
    },
    integrations: [
        starlight({
            head: [
                // Example: add Fathom analytics script tag.
                {
                    tag: "link",
                    attrs: {
                        href: "/downloaded/katex.min.css",
                        rel: "stylesheet",
                    },
                },
            ],
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
            social: [{ icon: "github", label: "GitHub", href: "https://github.com/Its-Just-Nans/yakb" }],
            sidebar: [
                {
                    label: "YakB",
                    collapsed: true,
                    autogenerate: { directory: "guides" },
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
