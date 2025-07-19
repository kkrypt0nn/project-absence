import React from "react";

type NodeDetailsProps = {
  title: string;
  children: React.ReactNode;
};

export default function NodeDetails({ title, children }: NodeDetailsProps) {
  return (
    <details
      style={{
        border: "1px",
        borderColor: "var(--ifm-details-border-color)",
        borderRadius: "6px",
        borderStyle: "solid",
      }}
      className="padding-vert--sm padding-horiz--md margin-top--md margin-bottom--md"
    >
      <summary>
        <strong>{title}</strong>
      </summary>
      <div className="margin-top--sm">{children}</div>
    </details>
  );
}
