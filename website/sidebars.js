// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  sidebar: [
    {
      type: "category",
      label: "Installation",
      items: [
        "installation/docker",
        "installation/cargo",
        "installation/from-source",
      ],
      collapsed: false,
    },
    {
      type: "category",
      label: "Usage",
      items: ["usage/arguments", "usage/config"],
      collapsed: true,
    },
    {
      type: "category",
      label: "Modules",
      items: [
        {
          type: "category",
          label: "Discovery",
          items: ["modules/discovery/subdomains"],
        },
        "modules/domain_takeover",
      ],
      collapsed: true,
    },
    {
      type: "category",
      label: "Scripting",
      items: ["scripting/basics"],
      collapsed: true,
    },
  ],
};

module.exports = sidebars;
