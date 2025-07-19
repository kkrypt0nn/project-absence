// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const { themes } = require("prism-react-renderer");
const lightCodeTheme = themes.oneLight;
const darkCodeTheme = themes.oneDark;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Project Absence",
  tagline: "👁️ Uncover the unseen",
  favicon: "assets/purple.png",

  url: "https://absence.krypton.ninja",
  baseUrl: "/",

  organizationName: "kkrypt0nn",
  projectName: "project-absence",
  trailingSlash: true,

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl:
            "https://github.com/kkrypt0nn/project-absence/tree/main/website",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
    [
      "@docusaurus/plugin-sitemap",
      {
        sitemap: {
          lastmod: "date",
          changefreq: "weekly",
          priority: 0.5,
          ignorePatterns: ["/tags/**"],
          filename: "sitemap.xml",
          createSitemapItems: async (params) => {
            const { defaultCreateSitemapItems, ...rest } = params;
            const items = await defaultCreateSitemapItems(rest);
            return items.filter((item) => !item.url.includes("/page/"));
          },
        },
      },
    ],
  ],

  plugins: [
    [
      "@docusaurus/plugin-content-docs",
      /** @type {import('@docusaurus/plugin-content-docs').Options} */
      ({
        id: "community",
        path: "community",
        routeBasePath: "community",
        sidebarPath: require.resolve("./sidebarsCommunity.js"),
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      metadata: [
        {
          name: "keywords",
          content: "project absence, information gathering, rust",
        },
        {
          name: "theme-color",
          content: "#11111b",
        },
      ],
      image: "assets/banner.png",
      colorMode: {
        defaultMode: "dark",
        disableSwitch: false,
        respectPrefersColorScheme: true,
      },
      announcementBar: {
        content:
          "⚠️ Project Absence is currently in <b>alpha version</b> and under ongoing development, meaning it may not be stable. This website is a work in progress as well!",
        backgroundColor: "var(--ifm-navbar-background-color)",
        textColor: "var(--ifm-font-color-base)",
        isCloseable: true,
      },
      navbar: {
        title: "Project Absence",
        logo: {
          alt: "Project Absence Logo",
          src: "/assets/purple.png",
        },
        items: [
          {
            to: "/graph-view",
            label: "Graph View",
            position: "right",
          },
          {
            type: "doc",
            docId: "README",
            label: "Documentation",
            position: "right",
          },
          {
            to: "/community/code-of-conduct",
            label: "Community",
            position: "right",
          },
          {
            href: "https://github.com/kkrypt0nn/project-absence",
            "aria-label": "GitHub",
            className: "header-github-link",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Project Absence",
            items: [
              {
                label: "Graph View",
                to: "/graph-view",
              },
              {
                label: "Documentation",
                to: "/docs",
              },
              {
                label: "Community",
                to: "/community/code-of-conduct",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "Code of Conduct",
                to: "/community/code-of-conduct",
              },
              {
                label: "Contributing Guidelines",
                to: "/community/contributing-guidelines",
              },
              {
                label: "Terms of Use",
                to: "/community/terms-of-use",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Project Absence (Made by Krypton)`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ["bash", "lua", "toml"],
      },
    }),
};

module.exports = config;
