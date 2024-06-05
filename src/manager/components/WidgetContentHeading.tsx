import { Button, Flex, Heading } from "@radix-ui/themes";

interface WidgetContentHeadingProps {
  heading: string;
  actionIcon: React.ReactNode;
  actionText: string;
  action: () => void;
}

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
