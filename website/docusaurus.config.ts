import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

const config: Config = {
  title: "Deskulpt",
  tagline: "A cross-platform desktop customization tool.",
  favicon: "img/favicon.ico",

  url: "https://CSCI-SHU-410-SE-Project.github.io/",
  baseUrl: "/Deskulpt/",
  organizationName: "CSCI-SHU-410-SE-Project",
  projectName: "Deskulpt",

  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  plugins: ["docusaurus-plugin-sass"],

  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          routeBasePath: "/",
          path: "./docs",
          sidebarPath: "./sidebars.ts",
        },
        theme: {
          customCss: ["./src/css/custom.scss"],
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: "img/docusaurus-social-card.jpg",
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: "Deskulpt",
      logo: {
        alt: "Deskulpt",
        src: "img/logo.svg",
        className: "invert-on-dark",
      },
      items: [
        {
          label: "Guides",
          sidebarId: "guides",
          type: "docSidebar",
        },
        {
          label: "Blog",
          to: "/blog",
        },
        {
          label: "GitHub",
          href: "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      copyright: `Copyright © ${new Date().getFullYear()} Deskulpt developers. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;