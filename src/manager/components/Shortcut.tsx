import { Flex, Kbd, KbdProps, Text } from "@radix-ui/themes";
import { ComponentPropsWithoutRef, Fragment, memo } from "react";

interface ShortcutProps {
  /** The array of shortcut keys. */
  keys: string[];
  /** The size of the shortcut display. */
  size?: KbdProps["size"];
}

/**
 * The shortcut display component.
 *
 * Each key will be displayed as a keyboard element, joint by a plus sign. They will be
 * wrapped in a flex box with default gap size 1. All flex box props can be passed to
 * this component to further customize the display layout.
 */
const Shortcut = memo(
  ({
    keys,
    size = "2",
    ...props
  }: ShortcutProps & ComponentPropsWithoutRef<typeof Flex>) => {
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
  },
);

export default Shortcut;
