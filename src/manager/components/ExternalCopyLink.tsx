import { Flex, FlexProps, IconButton, Link } from "@radix-ui/themes";
import { RxCopy } from "react-icons/rx";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { toast } from "sonner";
import { PropsWithChildren, useCallback } from "react";

interface Props {
  href: string;
  gap?: FlexProps["gap"];
}

export default ({ href, gap = "3", children }: PropsWithChildren<Props>) => {
  const handleCopy = useCallback(() => {
    writeText(href).then(() => toast.success("Copied to clipboard."));
  }, [href]);

  return (
    <Flex gap={gap} align="center">
      <Link href={href} target="_blank" rel="noreferrer">
        {children}
      </Link>
      <IconButton size="1" variant="ghost" onClick={handleCopy}>
        <RxCopy />
      </IconButton>
    </Flex>
  );
};
