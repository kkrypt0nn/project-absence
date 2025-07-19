import { checkFlag, DomainFlags } from "@site/src/utils/flags";
import { getPlatformBadge } from "@site/src/utils/platform-badge";
import Badge from "../Badge";
import NodeDetails from "../NodeDetails";

type DomainModalProps = {
  label: string;
  flags: number;
  possible_takeover_platform: string;
};

export default function DomainModal({
  label,
  flags,
  possible_takeover_platform,
}: DomainModalProps) {
  const expired = checkFlag(flags, DomainFlags.HAS_EXPIRED);
  const recent = checkFlag(flags, DomainFlags.IS_RECENT);
  const possible_takeover = checkFlag(flags, DomainFlags.POSSIBLE_TAKEOVER);

  return (
    <div className="container">
      <div className="row margin-top--md margin-bottom--md">
        <div className="col col--12">
          <div className="row">
            <div className="col col--4">
              <strong>Label:</strong>
            </div>
            <div className="col col--8">{label}</div>
          </div>
        </div>
      </div>
      <div className="row margin-top--md margin-bottom--md">
        <div className="col col--12">
          <div className="row">
            <div className="col col--4">
              <strong>Status:</strong>
            </div>
            <div className="col col--8">
              <Badge
                content={expired ? "Expired" : "Active"}
                type={expired ? "danger" : "success"}
              />
            </div>
          </div>
        </div>
      </div>
      <div className="row margin-top--md margin-bottom--md">
        <div className="col col--12">
          <div className="row">
            <div className="col col--4">
              <strong>Recent:</strong>
            </div>
            <div className="col col--8">
              <Badge
                content={recent ? "Yes" : "No"}
                type={recent ? "info" : "secondary"}
              />
            </div>
          </div>
        </div>
      </div>
      <div className="row margin-top--md margin-bottom--md">
        <div className="col col--12">
          <div className="row">
            <div className="col col--4">
              <strong>Possible Takeover:</strong>
            </div>
            <div className="col col--8">
              {possible_takeover ? (
                getPlatformBadge(possible_takeover_platform)
              ) : (
                <Badge content="No" type="danger" />
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
