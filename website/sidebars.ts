import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

export default {
  guides: [
    "guides/introduction",
    {
      type: "category",
      label: "Getting Started",
      link: {
        type: "doc",
        id: "guides/getting-started/index",
      },
      items: ["guides/getting-started/installation"],
    },
  ],
  api: ["api/overview"],
  development: [
    {
      type: "category",
      label: "Developer Guide",
      link: {
        type: "doc",
        id: "development/developer-guide/index",
      },
      items: [
        "development/developer-guide/contributing-code",
        "development/developer-guide/documentation",
      ],
    },
    {
      type: "category",
      label: "References",
      link: {
        type: "doc",
        id: "development/references/index",
      },
      items: [
        {
          type: "link",
          label: "Backend",
          href: "https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/",
          description: "Rust documentation for the Deskulpt backend.",
        },
        {
          type: "category",
          label: "Frontend",
          items: require("./docs/tsdoc/typedoc-sidebar.cjs"),
          description: "TypeScript documentation for the Deskulpt frontend.",
        },
      ],
    },
  ],
} satisfies SidebarsConfig;
