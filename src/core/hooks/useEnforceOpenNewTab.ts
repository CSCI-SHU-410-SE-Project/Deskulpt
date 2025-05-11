import { useEffect } from "react";

/**
 * Force all <a> elements to have attribute `target`  of value `_blank`.
 * This ensures all hyperlinks are opened in the default browser, instead of the webview window
 */
export function useEnforceOpenNewTab() {
  useEffect(() => {
    const handleClick = (event: MouseEvent) => {
      const link = event
        .composedPath()
        .find(
          (el) => el instanceof Node && el.nodeName.toUpperCase() === "A",
        ) as HTMLAnchorElement | undefined;

      if (link && link instanceof HTMLAnchorElement && link.href) {
        link.setAttribute("target", "_blank");
      }
    };

    document.addEventListener("click", handleClick);
    return () => document.removeEventListener("click", handleClick);
  }, []);
}
