import { Button, Flex, Heading } from "@radix-ui/themes";
import { ReactNode } from "react";

export interface WidgetContentHeadingProps {
  /** The component to put in the heading. */
  heading: ReactNode;
  /** The icon for the action button. */
  actionIcon: React.ReactNode;
  /** The text for the action button. */
  actionText: string;
  /** The action on button click. */
  action: () => void;
}

/**
 * The heading component for each section of a widget tab.
 *
 * This displays the heading aligned left and the action button aligned right. The
 * action button will be composed of the icon then the text.
 */
export default function WidgetContentHeading({
  heading,
  actionIcon,
  actionText,
  action,
}: WidgetContentHeadingProps) {
  return (
    <Flex justify="between" align="center">
      <Heading size="2">{heading}</Heading>
      <Button size="1" variant="surface" color="gray" highContrast onClick={action}>
        {actionIcon}
        {actionText}
      </Button>
    </Flex>
  );
}
