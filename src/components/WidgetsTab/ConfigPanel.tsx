import { Tooltip } from "antd";
import { Result, WidgetConfig } from "../../types/backend";
import * as styles from "./styles";

/**
 * The configuration information panel in the details of a widget.
 */
export default function ConfigInfoPanel(props: {
  config: Result<WidgetConfig, string>;
}) {
  const { config } = props;

  if ("Err" in config) {
    return (
      <div
        css={{
          height: "120px",
          paddingRight: "5px",
          fontFamily: "monospace",
          whiteSpace: "pre-wrap",
          overflowY: "auto",
          color: "red",
        }}
      >
        {config.Err}
      </div>
    );
  }

  const { deskulptConf, externalDeps } = config.Ok;
  return (
    <table>
      <tbody>
        <tr>
          <td css={styles.tableCaption}>Name</td>
          <td>{deskulptConf.name}</td>
        </tr>
        <tr>
          <td css={styles.tableCaption}>Entry</td>
          <td>
            <code>{deskulptConf.entry}</code>
          </td>
        </tr>
        <tr>
          <td css={styles.tableCaption}>Dependencies</td>
          <td>{getExternalDepsRepr(externalDeps)}</td>
        </tr>
      </tbody>
    </table>
  );
}

/**
 * Get the representation of the external dependencies.
 *
 * This component shows the number of external dependencies, and on float, a tooltip
 * showing the full list of dependencies.
 */
function getExternalDepsRepr(externalDeps: Record<string, string>) {
  const externalDepsEntries = Object.entries(externalDeps);
  if (externalDepsEntries.length === 0) {
    return "0";
  }

  const tooltipContent = (
    <div
      css={{
        maxHeight: "120px",
        maxWidth: "300px",
        overflow: "auto",
        scrollbarWidth: "none",
      }}
    >
      {externalDepsEntries.map(([dep, version], index) => (
        <div key={index}>
          <code>{dep}</code> {version}
        </div>
      ))}
    </div>
  );

  return (
    <Tooltip placement="right" title={tooltipContent}>
      {externalDepsEntries.length}
    </Tooltip>
  );
}
