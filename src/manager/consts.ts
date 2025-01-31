import { WidgetSettings } from "../types";

export const WEBSITE_URL =
  "https://csci-shu-410-se-project.github.io/Deskulpt/";
export const GITHUB_URL =
  "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/";

export const DEFAULT_WIDGET_SETTINGS: WidgetSettings = {
  x: 0,
  y: 0,
  opacity: 100,
};

export const KEY_CODE_MAPPING = {
  Digit0: "0",
  Digit1: "1",
  Digit2: "2",
  Digit3: "3",
  Digit4: "4",
  Digit5: "5",
  Digit6: "6",
  Digit7: "7",
  Digit8: "8",
  Digit9: "9",
  KeyA: "A",
  KeyB: "B",
  KeyC: "C",
  KeyD: "D",
  KeyE: "E",
  KeyF: "F",
  KeyG: "G",
  KeyH: "H",
  KeyI: "I",
  KeyJ: "J",
  KeyK: "K",
  KeyL: "L",
  KeyM: "M",
  KeyN: "N",
  KeyO: "O",
  KeyP: "P",
  KeyQ: "Q",
  KeyR: "R",
  KeyS: "S",
  KeyT: "T",
  KeyU: "U",
  KeyV: "V",
  KeyW: "W",
  KeyX: "X",
  KeyY: "Y",
  KeyZ: "Z",
};

export const MODIFIER_MAPPING = {
  Alt: "Alt",
  Ctrl: "Ctrl",
  Meta: "Super", // Tauri global shortcut plugin allows only "SUPER"
  Shift: "Shift",
};
