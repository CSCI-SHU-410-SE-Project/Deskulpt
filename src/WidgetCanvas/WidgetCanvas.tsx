import { useEffect } from "react";

import Widget from "../Widget/Widget";

import "./WidgetCanvas.css";
import { invoke } from "@tauri-apps/api/tauri";

const WidgetCanvas = () => {
  // Note: This shouldn't be a React state, because it's not a part of the component's state.
  const mouseDownInfo = {
    mouse_x: 0,
    mouse_y: 0,
    elem_left: 0,
    elem_top: 0,
    // activeWidget: null as HTMLElement | null,
  };

  const clip = function (value: number, min: number, max: number): number {
    if (max < min) {
      max = min;
    }
    return Math.min(Math.max(value, min), max);
  };

  const setElementToPos = function (
    elem: HTMLElement,
    container: HTMLElement,
    x: number,
    y: number,
  ): void {
    const new_left = clip(
      x,
      container.offsetLeft ? container.offsetLeft : 0,
      container.offsetWidth - elem.offsetWidth,
    );
    const new_top = clip(
      y,
      container.offsetTop ? container.offsetTop : 0,
      container.offsetHeight - elem.offsetHeight,
    );
    elem.style.left = `${new_left}px`;
    elem.style.top = `${new_top}px`;
  };

  useEffect(() => {
    const widgetCanvas = document.getElementById("widget-canvas")!;
    const widgets = document.getElementsByClassName(
      "widget",
    ) as HTMLCollectionOf<HTMLElement>;

    // make the active widget move as the mouse move
    widgetCanvas.addEventListener("mousemove", (e) => {
      e.preventDefault();
      // select the active widget by "active" pseudo-class
      const activeWidget = document.querySelector(
        ".widget:active",
      ) as HTMLElement | null;

      if (activeWidget === null) {
        return;
      }
      setElementToPos(
        activeWidget,
        widgetCanvas,
        // **Key idea**: move as mouse moves.
        //    At mousemove, the element position is calculated using
        //    - current mouse position
        //    - position of mouse down
        //    - position of element at mouse down
        e.clientX - mouseDownInfo.mouse_x + mouseDownInfo.elem_left,
        e.clientY - mouseDownInfo.mouse_y + mouseDownInfo.elem_top,
      );
    });

    widgetCanvas.addEventListener("mouseup", () => {
      mouseDownInfo.mouse_x = 0;
      mouseDownInfo.mouse_y = 0;
      mouseDownInfo.elem_left = 0;
      mouseDownInfo.elem_top = 0;
    });

    window.document.addEventListener("keydown", function (event) {
      // console.log('sss',event);
      if (event.key === "Escape") {
        // after pressing ESC
        invoke("sink_canvas");
      }
    });

    // initialize widget settings
    Array.from(widgets).forEach((widget) => {
      (widget.style.zIndex = "0"),
        widget.addEventListener("mousedown", (e) => {
          // on mousedown, store the position of mouse and widget
          mouseDownInfo.mouse_x = e.clientX;
          mouseDownInfo.mouse_y = e.clientY;
          mouseDownInfo.elem_left = widget.offsetLeft;
          mouseDownInfo.elem_top = widget.offsetTop;

          // on mouse down, bring the clicked widget to the top
          //  increment all widgets z-index by 1
          let max_z_index = 0;
          Array.from(widgets).forEach((widget) => {
            widget.style.zIndex = String(parseInt(widget.style.zIndex) + 1);
            max_z_index = Math.max(max_z_index, parseInt(widget.style.zIndex));
          });
          //  set the clicked widget z-index to max_z_index + 1
          widget.style.zIndex = String(max_z_index + 1);
          //  decrement all widgets z-index by 1
          Array.from(widgets).forEach((widget) => {
            widget.style.zIndex = String(parseInt(widget.style.zIndex) - 1);
          });
        });
    });
  }, []);

  return (
    <div id="widget-canvas" className="widget-canvas">
      {/* a for loop to create widgets with unique id 'widget-[number]' */}
      {[...Array(100).keys()].map((i) => {
        const styleConfig = {
          left: 10 + i * 10,
          top: 10 + (i % 10) * 50,
          width: 20,
          height: 10,
          color: "black",
          "background-color": `hsl(${i * 36}, 100%, 50%)`,
          "font-size": "16px",
          "font-weight": "bold",
          "border-radius": "5px",
          border: "2px solid black",
        };
        return <Widget key={i} label={`widget-${i}`} styleConfig={styleConfig} />;
      })}
    </div>
  );
};

export default WidgetCanvas;
