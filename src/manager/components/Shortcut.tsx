import { Flex, Kbd, Text } from "@radix-ui/themes";
import { ComponentPropsWithoutRef, Fragment } from "react";

interface ShortcutProps {
  keys: string[];
  size?: "1" | "2" | "3" | "4" | "5";
}

export default function Shortcut({
  keys,
  size = "2",
  ...props
}: ShortcutProps & ComponentPropsWithoutRef<typeof Flex>) {
  return (
    <Flex align="center" gap="1" {...props}>
      {keys.length > 0 ? (
        <Kbd size={size}>{keys[0]}</Kbd>
      ) : (
        <Text size={size}>None</Text>
      )}
      {keys.map(
        (key, index) =>
          index !== 0 && (
            <Fragment key={index}>
              <Text size={size}>+</Text>
              <Kbd size={size}>{key}</Kbd>
            </Fragment>
          ),
      )}
    </Flex>
  );
}
