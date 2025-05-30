<div align="center">

<a href="https://csci-shu-410-se-project.github.io/">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt-wide-dark.svg" />
    <img alt="Deskulpt" src="https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt-wide.svg" width="300px" />
  </picture>
</a>

**A cross-platform desktop customization tool.**

<br />

[![release](https://img.shields.io/github/v/release/CSCI-SHU-410-SE-Project/Deskulpt)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/latest) [![ci](https://img.shields.io/github/actions/workflow/status/CSCI-SHU-410-SE-Project/Deskulpt/ci.yaml?label=ci&logo=github)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/actions/workflows/ci.yaml) [![hompage](https://img.shields.io/badge/homepage-Deskulpt-blue.svg)](https://csci-shu-410-se-project.github.io/)

</div>

## Introduction

**Deskulpt** is a cross-platform desktop customization tool that allows you to write
React code to define widgets on your desktop. It is powered by [Tauri v2](https://v2.tauri.app/),
and written in Rust and TypeScript.

The project was started in 2024 as a final project for the Software Engineering course
at [NYU Shanghai](https://shanghai.nyu.edu/). It is currently maintained by the original
authors.

Similar softwares exist ([Rainmeter](https://www.rainmeter.net/) for Windows,
[Ubersicht](https://tracesof.net/uebersicht/) for MacOS), but what makes Deskulpt
different is that:

- **Deskulpt is cross-platform.** To the best of our knowledge, Deskulpt is the first
  desktop customization tool that supports all major desktop platforms (Windows, MacOS,
  Linux).

- **Deskulpt is light-weight and memory-safe.** Deskulpt is powered by Tauri, which uses
  Rust as its backend. Tauri provides a more light-weight solution than other existing
  frameworks (e.g. Electron), and Rust guarantees the memory safety of Deskulpt.

Currently, Deskulpt supports the following platforms:

- Windows (10, 11)
- MacOS
- Linux (X11, tested on Ubuntu)

Website: [https://csci-shu-410-se-project.github.io/](https://csci-shu-410-se-project.github.io/)

## Download

Download the latest release of Deskulpt [here](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases). See [Quick Start](https://csci-shu-410-se-project.github.io/guide/quick-start.html) for more details.

## Contributing

We welcome all sorts of contributions, no matter how large or small! See the [Contribution Guide](https://csci-shu-410-se-project.github.io/contribute/overview.html) for more details.
