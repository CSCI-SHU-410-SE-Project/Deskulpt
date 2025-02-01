import {
  Flex,
  IconButton,
  Link,
  Popover,
  ScrollArea,
  Text,
  Tooltip,
} from "@radix-ui/themes";
import { LuView } from "react-icons/lu";

export interface WidgetDependenciesProps {
  /** The dependencies mapping package name to the corresponding version string. */
  dependencies: Record<string, string>;
}

/**
 * Display component for widget dependencies.
 *
 * This component normally just displays the number of dependencies. It also renders
 * a view details button, which on click will show a popover with a list of package
 * names and corresponding version strings. The package names are linked to their URL
 * on [npmjs.com](https://www.npmjs.com/).
 */
export default function WidgetDependencies({
  dependencies,
}: WidgetDependenciesProps) {
  const dependenciesArray = Object.entries(dependencies);

  return (
    <Flex gap="2" align="center">
      {dependenciesArray.length > 0 ? (
        <Popover.Root>
          <Tooltip content="View details" side="right">
            <Popover.Trigger>
              <IconButton variant="ghost" size="1">
                <LuView />
              </IconButton>
            </Popover.Trigger>
          </Tooltip>
          <Popover.Content size="1">
            <ScrollArea scrollbars="vertical">
              <Flex direction="column" maxHeight="100px" pr="4" gap="1">
                {dependenciesArray.map(([name, version]) => (
                  <Flex key={name} gap="4" align="center">
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
      ) : (
        "None"
      )}
    </Flex>
  );
}
