import { Button, Flex, Heading } from "@radix-ui/themes";

interface Props {
  heading: string;
  actionIcon: React.ReactNode;
  actionText: string;
  action: () => void;
}

/**
 * The heading component for each section of a widget tab.
 *
 * This displays the heading aligned left and the action button aligned right. The
 * action button will be composed of the icon then the text.
 */
export default ({ heading, actionIcon, actionText, action }: Props) => {
  return (
    <Flex justify="between" align="center">
      <Heading size="2">{heading}</Heading>
      <Button
        size="1"
        variant="surface"
        color="gray"
        highContrast
        onClick={action}
      >
        {actionIcon}
        {actionText}
      </Button>
    </Flex>
  );
};
