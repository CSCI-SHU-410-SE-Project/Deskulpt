# Introduction

## What is Deskulpt?

**Deskulpt** is a free, modern, and cross-platform desktop customization tool designed to transform your desktop experience. It allows you to create, run, and manage widgets directly on your desktop.

### Deskulpt Widgets

A **Deskulpt widget** is a small, interactive web component rendered on the Deskulpt canvas. Widgets can range from simple, single-purpose tools such as clocks, weather forecasts, or system monitoring displays, to more complex mini-applications with rich interaction and detailed information such as an interactive calendar. Deskulpt widgets can be freely moved and arranged on your desktop, personalizing your workspace exactly as you like.

Deskulpt widgets are built with web technologies, primarily React, JavaScript, and TypeScript. This allows widgets to leverage the full power and flexibility of modern web development. Anything you can build as a web application can become a Deskulpt widget, empowering you to bring advanced functionalities directly onto your desktop. Widgets can seamlessly gather data from your system, web APIs, local files, etc., and dynamically present data using rich and interactive visual elements and user interfaces.

### Deskulpt Canvas

The **Deskulpt canvas** is a transparent webview window layered on your desktop, always staying below your other application windows. The Deskulpt canvas has two modes: **sink mode** and **float mode**.

- In sink mode, widgets are not user-interactable, though they are still dynamic. You can use your desktop as normal, as if the widgets are just small dynamic wallpapers.
- In float mode, widgets are fully user-interactable, meaning that they are responsive to clicks, keyboard events, etc. and you can drag them around. However you cannot interact with your desktop, e.g., you cannot click desktop icons.

This means you cannot interact with your widgets and the desktop at the same time, but you can easily switch between these two modes to gain the best experience.

## Why Deskulpt?

:::warning 🚧 Under Construction 🚧
We are working on this part of the documentation.
:::

## Do I Need Coding Skills?

If you simply want to use widgets from Deskulpt's widget library as is, you don't need any coding skills. Deskulpt provides an intuitive user interface to easily download and manage widgets, customize their positions, transparency, visibility, and other basic settings.

Most Deskulpt widgets include some easy-to-use and well-documented controls that anyone can also understand. You can customize basic aspects of widgets, such as text, colors, and appearance, without needing any coding background.

For deeper customizations or creating full widgets, familiarity with JavaScript, TypeScript, or React is beneficial, though not mandatory. Deskulpt provides clear documentation and community support to help users of all skill levels. For widget developers who aim to build widgets with even more advanced functionalities that require external dependencies or custom plugins, additional tools like npm and more advanced coding skills may be needed.
