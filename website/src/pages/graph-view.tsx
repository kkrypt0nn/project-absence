import BrowserOnly from "@docusaurus/BrowserOnly";
import { faFlask } from "@fortawesome/free-solid-svg-icons";
import Admonition from "@theme/Admonition";
import CodeBlock from "@theme/CodeBlock";
import Layout from "@theme/Layout";
import "@xyflow/react/dist/style.css";
import { useEffect, useRef, useState } from "react";
import Badge from "../components/Badge";
import Graph from "../components/Graph";

const EXAMPLE_DATA = {
  type: "domain",
  value: "krypton.ninja",
  connections: [
    {
      type: "domain",
      value: "status.krypton.ninja",
      connections: [],
      data: {
        flags: 0,
      },
    },
    {
      type: "domain",
      value: "beta.krypton.ninja",
      connections: [],
      data: {
        flags: 2,
      },
    },
    {
      type: "domain",
      value: "github.krypton.ninja",
      connections: [],
      data: {
        flags: 4,
        possible_takeover: "github",
      },
    },
    {
      type: "email",
      value: "root@krypton.ninja",
      connections: [],
      data: {},
    },
  ],
  data: {},
};

export default function GraphView() {
  const [data, setData] = useState(null);
  const [error, setError] = useState("");

  const hiddenFileInput = useRef(null);
  const handleClick = (_) => {
    hiddenFileInput.current.click();
  };
  const handleChange = (event) => {
    const fileUploaded: File = event.target.files[0];
    if (fileUploaded.type != "application/json") {
      setError("The file must be a JSON file!");
      return;
    }
    const fileReader = new FileReader();
    fileReader.readAsText(fileUploaded, "UTF-8");
    fileReader.onload = (event) => {
      try {
        let parsed = JSON.parse(event.target.result.toString());
        setData(parsed);
        setError("");
        return;
      } catch (SyntaxError) {
        setError("Invalid JSON data provided!");
        return;
      }
    };
  };

  const useExampleData = (_) => {
    setData(EXAMPLE_DATA);
    setError("");
    return;
  };

  useEffect(() => {
    function onKeyPressed(event: KeyboardEvent) {
      if (event.key == "v" && (event.metaKey || event.ctrlKey)) {
        navigator.clipboard
          .readText()
          .then((content: string) => {
            try {
              let parsed = JSON.parse(content);
              setData(parsed);
              setError("");
              document.removeEventListener("keydown", onKeyPressed);
              return;
            } catch (SyntaxError) {
              setError("Invalid JSON data provided!");
              return;
            }
          })
          .catch(() => {});
      }
    }
    document.addEventListener("keydown", onKeyPressed);
  }, []);

  return (
    <Layout
      title="Graph View"
      description="Get a graph view of the discovered information."
    >
      <main>
        <div className="container padding-top--md padding-bottom--lg">
          <div className="markdown">
            <h1>
              Graph View <Badge content="BETA" icon={faFlask} />
            </h1>
            {error && (
              <Admonition type="danger" title="Error">
                <p>{error}</p>
              </Admonition>
            )}
            <p>
              The Graph View is in <strong>beta</strong> hence does not support
              everything the tool may support. Currently it will only display
              nodes of type <code>domain</code> and not the others (such as{" "}
              <code>email</code> and <code>file</code>).
            </p>
            {data ? (
              <>
                <div style={{ height: "75vh", width: "100%" }}>
                  <Graph data={data} />
                </div>
                <details>
                  <summary>JSON Data:</summary>
                  <CodeBlock language="json" showLineNumbers>
                    {JSON.stringify(data, null, 4)}
                  </CodeBlock>
                </details>
              </>
            ) : (
              <>
                <p>
                  <button
                    className="button button--primary"
                    onClick={handleClick}
                    type="submit"
                  >
                    Select JSON file
                  </button>
                  <input
                    type="file"
                    onChange={handleChange}
                    ref={hiddenFileInput}
                    style={{ display: "none" }}
                  />
                  {process.env.NODE_ENV === "development" && (
                    <button
                      className="button button--secondary button--outline button--sm margin--md"
                      onClick={useExampleData}
                    >
                      Use example data
                    </button>
                  )}
                </p>
                <BrowserOnly>
                  {() =>
                    !navigator.userAgent.match("iPhone|iPad|Android") && (
                      <small>
                        On a computer you can also press{" "}
                        <kbd>
                          {navigator.userAgent.includes("Mac") ? "Cmd" : "Ctrl"}
                        </kbd>{" "}
                        + <kbd>V</kbd> to use the content of your clipboard.
                      </small>
                    )
                  }
                </BrowserOnly>
              </>
            )}
          </div>
        </div>
      </main>
    </Layout>
  );
}
