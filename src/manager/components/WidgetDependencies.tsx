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

interface WidgetDependenciesProps {
  dependencies: Record<string, string>;
}

export default function WidgetDependencies({ dependencies }: WidgetDependenciesProps) {
  const dependenciesArray = Object.entries(dependencies);

  return (
    <Flex gap="2" align="center">
      {dependenciesArray.length || "None"}
      {dependenciesArray.length !== 0 && (
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
                {dependenciesArray.map(([name, version], index) => (
                  <Flex key={index} gap="4" align="center">
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
      )}
    </Flex>
  );
}
