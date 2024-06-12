import {
  Button,
  Flex,
  Link,
  Popover,
  ScrollArea,
  Separator,
  Text,
} from "@radix-ui/themes";
import { invokeBundleExternalDependencies } from "../../commands";
import { useState } from "react";

export interface WidgetDependenciesProps {
  /** The widget ID. */
  widgetId: string;
  /** The dependencies mapping package name to the corresponding version string. */
  dependencies: Record<string, string>;
}

/**
 * Display component for widget dependencies.
 *
 * If there is no external dependency this will just display "None". Otherwise, it will
 * display a "details" button and a "bundle" button. The "details" button will show a
 * popover with the list of package names and corresponding version strings. The
 * "bundle" button is used to bundle the external dependencies.
 */
export default function WidgetDependencies({
  widgetId,
  dependencies,
}: WidgetDependenciesProps) {
  const [isBundlingExternal, setIsBundlingExternal] = useState(false);
  const dependenciesArray = Object.entries(dependencies);

  return (
    <Flex gap="3" align="center">
      {dependenciesArray.length !== 0 ? `Total ${dependenciesArray.length}` : "None"}
      {dependenciesArray.length !== 0 && (
        <>
          <Separator orientation="vertical" />
          <Popover.Root>
            <Popover.Trigger>
              <Button size="2" variant="ghost" css={{ height: "var(--space-3)" }}>
                View
              </Button>
            </Popover.Trigger>
            <Popover.Content size="1" maxWidth="300px">
              <ScrollArea scrollbars="vertical">
                <Flex direction="column" maxHeight="100px" pr="4" gap="1">
                  {dependenciesArray.map(([name, version], index) => (
                    <Flex key={index} align="center" gap="5">
                      <Link
                        size="1"
                        href={`https://www.npmjs.com/package/${name}`}
                        target="_blank"
                        rel="noreferrer"
                      >
                        {name}
                      </Link>
                      <Text size="1">{version}</Text>
                    </Flex>
                  ))}
                </Flex>
              </ScrollArea>
            </Popover.Content>
          </Popover.Root>
          <Separator orientation="vertical" />
          <Button
            size="2"
            variant="ghost"
            loading={isBundlingExternal}
            css={{ height: "var(--space-3)" }}
            onClick={() => {
              setIsBundlingExternal(true);
              invokeBundleExternalDependencies(widgetId)
                .catch(console.error)
                .finally(() => setIsBundlingExternal(false));
            }}
          >
            Bundle
          </Button>
        </>
      )}
    </Flex>
  );
}
