import { Flex, Kbd, KbdProps, Text } from "@radix-ui/themes";
import { ComponentPropsWithoutRef, Fragment } from "react";

interface Props {
  keys: string[];
  size?: KbdProps["size"];
}

export default ({
  keys,
  size = "2",
  ...props
}: Props & ComponentPropsWithoutRef<typeof Flex>) => {
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
            <Fragment key={key}>
              <Text size={size}>+</Text>
              <Kbd size={size}>{key}</Kbd>
            </Fragment>
          ),
      )}
    </Flex>
  );
};
