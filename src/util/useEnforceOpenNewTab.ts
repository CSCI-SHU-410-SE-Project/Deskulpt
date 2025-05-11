import { openUrl } from "@tauri-apps/plugin-opener";

/**
 * Adapted from [the init script of Tauri's Opener plugin](https://github.com/tauri-apps/plugins-workspace/blob/a6b854032d0b10f0f17c4ffa6bdf4a05429e05fb/plugins/opener/guest-js/init.ts)
 *
 * Force all `<a>` elements to have attribute `target`  of value `_blank`.
 * This ensures all hyperlinks are opened in the default browser, instead of the webview window
 */
export function enforceOpenNewTab() {
  const handleClick = (event: MouseEvent) => {
    // return early if
    if (
      // event was prevented
      event.defaultPrevented ||
      // or not a left click
      event.button !== 0 ||
      // or meta key pressed
      event.metaKey ||
      // or al key pressed
      event.altKey
    )
      return;

    const a = event
      .composedPath()
      .find((el) => el instanceof Node && el.nodeName.toUpperCase() === "A") as
      | HTMLAnchorElement
      | undefined;

    // return early if
    if (
      // not tirggered from <a> element
      !a ||
      // or doesn't have a href
      !a.href
      // or not supposed to be open in a new tab
      // || !(
      // open new tab whether or not target is _blank
      // a.target === '_blank'
      // or ctrl key pressed
      // event.ctrlKey
      // or shift key pressed
      // || event.shiftKey
      // )
    )
      return;

    const url = new URL(a.href);

    // return early if
    if (
      // same origin (internal navigation)
      url.origin === window.location.origin ||
      // not default protocols
      ["http:", "https:", "mailto:", "tel:"].every((p) => url.protocol !== p)
    )
      return;

    event.preventDefault();

    // void invoke("plugin:opener|open_url", {
    //   url,
    // });
    openUrl(url.toString());
  };
  document.addEventListener("click", handleClick);
}
