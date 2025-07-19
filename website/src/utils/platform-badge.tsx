import { faGithub } from "@fortawesome/free-brands-svg-icons";
import {
  faBug,
  faCaretUp,
  faH,
  faHexagonNodes,
  faR,
  faTrain,
} from "@fortawesome/free-solid-svg-icons";
import Badge from "../components/Badge";

export function getPlatformBadge(platform: string) {
  switch (platform) {
    case "github":
      return <Badge content="GitHub" icon={faGithub} type="secondary" />;
    case "glitch":
      return <Badge content="Glitch" icon={faBug} type="secondary" />;
    case "heroku":
      return <Badge content="Heroku" icon={faH} type="secondary" />;
    case "netlify":
      return <Badge content="Netlify" icon={faHexagonNodes} type="secondary" />;
    case "railway":
      return <Badge content="Railway" icon={faTrain} type="secondary" />;
    case "replit":
      return <Badge content="Replit" icon={faR} type="secondary" />;
    case "vercel":
      return <Badge content="Vercel" icon={faCaretUp} type="secondary" />;
    default:
      return <Badge content={`Yes (${platform})`} type="success" />;
  }
}
