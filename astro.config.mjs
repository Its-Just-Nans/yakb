// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import rehypeKatex from "rehype-katex";
import rehypeExternalLinks from "rehype-external-links";
import remarkMath from "remark-math";
import remarkToc from "remark-toc";

// https://astro.build/config
export default defineConfig({
    site: "https://yakb.n4n5.dev",
    markdown: {
        remarkPlugins: [remarkMath, remarkToc],
        rehypePlugins: [rehypeKatex, [rehypeExternalLinks, { target: "_blank" }]],
    },
    integrations: [
        starlight({
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
                    autogenerate: { directory: "guides" },
                },
            ],
            customCss: ["./src/styles/custom.css", "./src/styles/katex.min.css"],
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
