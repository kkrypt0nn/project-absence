import {
  faApple,
  faDocker,
  faLinux,
  faWindows,
} from "@fortawesome/free-brands-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Layout from "@theme/Layout";

import { config } from "@fortawesome/fontawesome-svg-core";
import "@fortawesome/fontawesome-svg-core/styles.css";
import { faFlask } from "@fortawesome/free-solid-svg-icons";
import Badge from "../components/Badge";
config.autoAddCss = false;

export default function Home() {
  // TODO: Write this entire page better
  return (
    <Layout title={"Project Absence"} description="👁️ Uncover the unseen">
      <main>
        <div className="container padding-top--md padding-bottom--lg">
          <div style={{ textAlign: "center" }} className="markdown">
            <h1>Project Absence</h1>
            <img src="/assets/purple.png" height="20%" width="20%" />
            <p>
              <strong>👁️ Uncover the unseen</strong>
            </p>
            <p>
              Project Absence is a domain and server OSINT tool for system
              administrators and security engineers. It currently supports
              subdomain discovery and detection of potential domain takeover
              opportunities on common hosting platforms.
            </p>
            <p>
              To maintain an OSINT-only approach, the tool contacts each
              discovered domain or server only once, to extract information
              based on the returned content - valuable data that can help
              and lead to further discoveries.
            </p>
            <p>More features are in development.</p>
            <div className="margin-bottom--xl">
              <a href="/docs" className="button button--primary">
                Documentation
              </a>
              <a
                href="/graph-view"
                className="button button--secondary button--outline margin-left--md"
              >
                View tool results as graph{" "}
                <Badge content="BETA" icon={faFlask} />
              </a>
            </div>
            <div
              style={{
                display: "flex",
                gap: "20px",
                justifyContent: "center",
                alignItems: "center",
              }}
            >
              <FontAwesomeIcon size="3x" icon={faLinux} />
              <FontAwesomeIcon size="3x" icon={faApple} />
              <FontAwesomeIcon size="3x" icon={faWindows} />
              <FontAwesomeIcon size="3x" icon={faDocker} />
            </div>
          </div>
        </div>
      </main>
    </Layout>
  );
}
