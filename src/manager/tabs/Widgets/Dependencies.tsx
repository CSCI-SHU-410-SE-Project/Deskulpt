import { css } from "@emotion/react";
import { Code, Dialog, Inset, Link, Table } from "@radix-ui/themes";
import { memo, useMemo } from "react";

const styles = {
  table: css({ "[data-radix-scroll-area-viewport]": { maxHeight: "300px" } }),
};

interface Props {
  deps: Record<string, string>;
}

const Dependencies = memo(({ deps }: Props) => {
  const depsArray = useMemo(() => Object.entries(deps), [deps]);

  return depsArray.length > 0 ? (
    <Dialog.Root>
      <Dialog.Trigger>
        <Link size="2" href="#">
          View ({depsArray.length})
        </Link>
      </Dialog.Trigger>
      <Dialog.Content size="2" width="400px" aria-describedby={undefined}>
        <Dialog.Title size="2">Dependencies</Dialog.Title>
        <Inset side="x">
          <Table.Root size="1" css={styles.table}>
            <Table.Body>
              {depsArray.map(([name, version]) => (
                <Table.Row key={name}>
                  <Table.Cell>
                    <Link
                      href={`https://www.npmjs.com/package/${name}`}
                      target="_blank"
                      rel="noreferrer"
                    >
                      <Code variant="ghost">{name}</Code>
                    </Link>
                  </Table.Cell>
                  <Table.Cell>
                    <Code variant="ghost">{version}</Code>
                  </Table.Cell>
                </Table.Row>
              ))}
            </Table.Body>
          </Table.Root>
        </Inset>
      </Dialog.Content>
    </Dialog.Root>
  ) : (
    "None"
  );
});

export default Dependencies;
