import {
  faAngular,
  faCpanel,
  faWindows,
  faWolfPackBattalion,
} from "@fortawesome/free-brands-svg-icons";
import {
  faAddressBook,
  faCircleQuestion,
  faCube,
  faDatabase,
  faDesktop,
  faEnvelope,
  faEnvelopeOpen,
  faFile,
  faFileShield,
  faGlobe,
  faHashtag,
  faLayerGroup,
  faLock,
  faM,
  faMountain,
  faPrint,
  faServer,
  faShield,
  faTerminal,
  faWifi,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

const ICON_SIZE = "lg";

export function getServiceIcon(service: string) {
  switch (service) {
    case "ftp":
      return <FontAwesomeIcon icon={faFile} size={ICON_SIZE} />;
    case "ssh":
      return <FontAwesomeIcon icon={faTerminal} size={ICON_SIZE} />;
    case "telnet":
      return <FontAwesomeIcon icon={faTerminal} size={ICON_SIZE} />;
    case "smtp":
      return <FontAwesomeIcon icon={faEnvelope} size={ICON_SIZE} />;
    case "whois":
      return <FontAwesomeIcon icon={faCircleQuestion} size={ICON_SIZE} />;
    case "dns":
      return <FontAwesomeIcon icon={faAddressBook} size={ICON_SIZE} />;
    case "http":
      return <FontAwesomeIcon icon={faGlobe} size={ICON_SIZE} />;
    case "kerberos-auth":
      return <FontAwesomeIcon icon={faWolfPackBattalion} size={ICON_SIZE} />;
    case "pop3":
      return <FontAwesomeIcon icon={faEnvelopeOpen} size={ICON_SIZE} />;
    case "sftp":
      return <FontAwesomeIcon icon={faFileShield} size={ICON_SIZE} />;
    case "imap":
      return <FontAwesomeIcon icon={faEnvelopeOpen} size={ICON_SIZE} />;
    case "ldap":
      return <FontAwesomeIcon icon={faWindows} size={ICON_SIZE} />;
    case "https":
      return <FontAwesomeIcon icon={faLock} size={ICON_SIZE} />;
    case "kerberos-password":
      return <FontAwesomeIcon icon={faWolfPackBattalion} size={ICON_SIZE} />;
    case "cups":
      return <FontAwesomeIcon icon={faPrint} size={ICON_SIZE} />;
    case "kerberos-admin":
      return <FontAwesomeIcon icon={faWolfPackBattalion} size={ICON_SIZE} />;
    case "ftps":
      return <FontAwesomeIcon icon={faFileShield} size={ICON_SIZE} />;
    case "imaps":
      return <FontAwesomeIcon icon={faEnvelope} size={ICON_SIZE} />;
    case "pop3s":
      return <FontAwesomeIcon icon={faEnvelope} size={ICON_SIZE} />;
    case "openvpn":
      return <FontAwesomeIcon icon={faShield} size={ICON_SIZE} />;
    case "wifi-pineapple":
      return <FontAwesomeIcon icon={faWifi} size={ICON_SIZE} />;
    case "cpanel":
      return <FontAwesomeIcon icon={faCpanel} size={ICON_SIZE} />;
    case "valheim":
      return <FontAwesomeIcon icon={faMountain} size={ICON_SIZE} />;
    case "mysql":
      return <FontAwesomeIcon icon={faDatabase} size={ICON_SIZE} />;
    case "rdp":
      return <FontAwesomeIcon icon={faDesktop} size={ICON_SIZE} />;
    case "angular":
      return <FontAwesomeIcon icon={faAngular} size={ICON_SIZE} />;
    case "metasploit":
      return <FontAwesomeIcon icon={faM} size={ICON_SIZE} />;
    case "postgresql":
      return <FontAwesomeIcon icon={faDatabase} size={ICON_SIZE} />;
    case "vnc":
      return <FontAwesomeIcon icon={faDesktop} size={ICON_SIZE} />;
    case "redis":
      return <FontAwesomeIcon icon={faLayerGroup} size={ICON_SIZE} />;
    case "irc":
      return <FontAwesomeIcon icon={faHashtag} size={ICON_SIZE} />;
    case "http-proxy":
      return <FontAwesomeIcon icon={faServer} size={ICON_SIZE} />;
    case "minecraft":
      return <FontAwesomeIcon icon={faCube} size={ICON_SIZE} />;
    default:
      return null;
  }
}
