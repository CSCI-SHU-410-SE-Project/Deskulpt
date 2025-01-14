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

export interface ExternalCopyLinkProps {
  /** The external target of the link. */
  href: LinkProps["href"];
  /** The gap size between the link text and the copy button. */
  gap?: FlexProps["gap"];
}

/**
 * The external link component with a copy button.
 *
 * The link will be opened in a new tab when clicked. The copy button will copy the link
 * address to the clipboard. Wrap the link text within this component.
 */
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
            writeText(
              "https://csci-shu-410-se-project.github.io/Deskulpt/",
            ).then(() => toast.success("Copied to clipboard."))
          }
        >
          <RxCopy />
        </IconButton>
      </Tooltip>
    </Flex>
  );
}
