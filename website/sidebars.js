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
          items: [
            "modules/discovery/emails",
            "modules/discovery/endpoints",
            "modules/discovery/files",
            "modules/discovery/subdomains",
          ],
        },
        "modules/dns",
        "modules/domain_takeover",
        "modules/infrastructure",
      ],
      collapsed: true,
    },
    {
      type: "category",
      label: "Tutorials",
      items: ["tutorials/first_script"],
      collapsed: true,
    },
    {
      type: "category",
      label: "Scripting",
      items: [
        "scripting/basics",
        "scripting/events",
        "scripting/session",
        "scripting/database",
        "scripting/globals",
      ],
      collapsed: true,
    },
  ],
};

module.exports = sidebars;
