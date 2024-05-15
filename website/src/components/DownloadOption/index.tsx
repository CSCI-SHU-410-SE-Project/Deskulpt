import Link from "@docusaurus/Link";

export default function DownloadOption(props: {
  version: string;
  artifact: string;
  children: JSX.Element;
}) {
  const { version, artifact, children } = props;

  return (
    <div
      className="container"
      style={{
        margin: "10px 0",
        display: "flex",
        alignItems: "center",
        justifyContent: "space-between",
      }}
    >
      <span>{children}</span>
      <Link
        className="button button--secondary button--sm"
        href={`https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/download/v${version}/${artifact}`}
        title={artifact}
      >
        Download
      </Link>
    </div>
  );
}
