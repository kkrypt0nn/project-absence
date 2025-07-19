import { IconDefinition } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

type BadgeProps = {
  content: string;
  type?:
    | "primary"
    | "secondary"
    | "success"
    | "info"
    | "warning"
    | "danger"
    | undefined;
  icon?: IconDefinition;
};

export default function Badge({ content, type, icon }: BadgeProps) {
  return (
    <span className={`badge badge--${type === undefined ? "primary" : type}`}>
      {icon && <FontAwesomeIcon icon={icon}></FontAwesomeIcon>} {content}
    </span>
  );
}
