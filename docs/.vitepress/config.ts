import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Deskulpt",
  description: "A cross-platform desktop customization tool.",
  base: "/Deskulpt/",
  srcDir: "src",
  themeConfig: {
    logo: {
      light: "/deskulpt.svg",
      dark: "/deskulpt-dark.svg",
    },
    nav: [
      { text: "User Guide", link: "/guide/introduction" },
      { text: "Contribute", link: "/contribute/overview" },
    ],
    sidebar: {
      "/guide/": [
        {
          text: "Getting Started",
          items: [
            { text: "Introduction", link: "/guide/introduction" },
            { text: "Quick Start", link: "/guide/quick-start" },
          ],
        },
      ],
      "/contribute/": [
        {
          text: "Contribution Guide",
          items: [
            { text: "Overview", link: "/contribute/overview" },
            { text: "Build and Run", link: "/contribute/build-and-run" },
            { text: "Testing", link: "/contribute/testing" },
            {
              text: "Quality Assurance",
              link: "/contribute/quality-assurance",
            },
            { text: "Documentation", link: "/contribute/documentation" },
          ],
        },
        {
          text: "Developers' Reference",
          items: [
            {
              text: "Backend Rustdoc",
              link: "https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/",
            },
          ],
        },
      ],
    },
    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt",
      },
    ],
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2023-2025 The Deskulpt Development Team",
    },
    lastUpdated: {},
    search: {
      provider: "local",
    },
  },
});
