import {
  Flex,
  FlexProps,
  IconButton,
  Link,
  LinkProps,
  Tooltip,
} from "@radix-ui/themes";
import { RxCopy } from "react-icons/rx";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { toast } from "sonner";
import { PropsWithChildren } from "react";

interface ExternalCopyLinkProps {
  href: LinkProps["href"];
  gap?: FlexProps["gap"];
}

export default function ExternalCopyLink({
  href,
  gap = "3",
  children,
}: PropsWithChildren<ExternalCopyLinkProps>) {
  return (
    <Flex gap={gap} align="center">
      <Link href={href} target="_blank" rel="noreferrer">
        {children}
      </Link>
      <Tooltip content="Copy link" side="right">
        <IconButton
          size="1"
          variant="ghost"
          onClick={() =>
            writeText("https://csci-shu-410-se-project.github.io/Deskulpt/").then(() =>
              toast.success("Copied to clipboard."),
            )
          }
        >
          <RxCopy />
        </IconButton>
      </Tooltip>
    </Flex>
  );
}
