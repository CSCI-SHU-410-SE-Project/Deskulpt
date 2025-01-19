import { useEffect, useState } from "react";

export default function usePlatform(defaultPlatform: string) {
  const [platform, setPlatform] = useState(defaultPlatform);

  useEffect(() => {
    const userAgent = window.navigator.userAgent.toLowerCase();
    if (userAgent.indexOf("win") !== -1) {
      setPlatform("windows");
    } else if (userAgent.indexOf("mac") !== -1) {
      setPlatform("macos");
    } else if (
      userAgent.indexOf("linux") !== -1 ||
      userAgent.indexOf("x11") !== -1
    ) {
      setPlatform("linux");
    }
  }, []);

  return platform;
}
