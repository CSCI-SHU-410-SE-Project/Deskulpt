import { useEffect } from "react";

import "./Widget.css";

// interface of widget style config
interface StyleConfig {
  left: number;
  top: number;
  // custom style properties
  [key: string]: any;
}

const Widget = ({
  label,
  styleConfig,
}: {
  label: string;
  styleConfig: StyleConfig;
}) => {
  const stylizeWidget = (widget: HTMLElement, styleConfig: StyleConfig) => {
    widget.style.left = `${styleConfig.left}px`;
    widget.style.top = `${styleConfig.top}px`;
    // set custom style properties
    for (const key in styleConfig) {
      if (key === "left" || key === "top") {
        continue;
      }
      widget.style.setProperty(key, styleConfig[key]);
    }
  };

  useEffect(() => {
    // set (restore) the style of the widget when it's mounted
    const widget = document.getElementById(label) as HTMLElement | null;
    if (widget === null) {
      console.error(`Widget with label ${label} not found`);
      return;
    }
    stylizeWidget(widget, styleConfig);
    // set the style of the widget using React.CSSProperties
  }, []);

  return (
    <div className="widget" id={label}>
      {label}
    </div>
  );
};

export default Widget;
