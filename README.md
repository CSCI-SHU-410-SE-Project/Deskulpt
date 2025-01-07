<div align="center">

<a href="https://csci-shu-410-se-project.github.io/Deskulpt/">
  <img id="deskulpt-logo" alt="Deskulpt" src="https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt-wide.svg" width="300px" />
</a>

**A cross-platform desktop customization tool.**

<br />

|            |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Project    | [![status](https://img.shields.io/badge/status-alpha-red.svg)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt) [![license](https://img.shields.io/github/license/CSCI-SHU-410-SE-Project/Deskulpt)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/blob/main/LICENSE.md) [![hompage](https://img.shields.io/badge/homepage-Deskulpt-blue.svg)](https://csci-shu-410-se-project.github.io/Deskulpt/)                                                                                                                                                                                                                                                        |
| Repository | [![build](https://img.shields.io/github/actions/workflow/status/CSCI-SHU-410-SE-Project/Deskulpt/build.yaml?label=build&logo=github)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/actions/workflows/build.yaml) [![test-core](https://img.shields.io/github/actions/workflow/status/CSCI-SHU-410-SE-Project/Deskulpt/test-core.yaml?label=test%20core&logo=github)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/actions/workflows/test-core.yaml)                                                                                                                                                                                                    |
| Release    | [![release](https://img.shields.io/github/v/release/CSCI-SHU-410-SE-Project/Deskulpt)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/latest) [![windows](https://img.shields.io/badge/Windows-000000?style=flat&logo=windows&logoColor=white)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/latest) [![macos](https://img.shields.io/badge/MacOS-000000?style=flat&logo=macos&logoColor=white)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/latest) [![linux](https://img.shields.io/badge/Linux-000000?style=flat&logo=macos&logoColor=white)](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases/latest) |

</div>

## Introduction

**Deskulpt** is a cross-platform desktop customization tool that allows you to write
React code to define widgets on your desktop. It is powered by [Tauri](https://tauri.app/),
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

Website: [https://csci-shu-410-se-project.github.io/Deskulpt/](https://csci-shu-410-se-project.github.io/Deskulpt/)

## Download

Download the latest release of Deskulpt [here](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/releases).
To install and build Deskulpt from source, see [development](#development). Refer to the
[installation guide](https://csci-shu-410-se-project.github.io/Deskulpt/guides/getting-started/installation)
for more details.

## Development

We welcome new contributors of all experience levels. The goals of the Deskulpt
community are to be helpful, welcoming, and effective. The [developers' guide](https://csci-shu-410-se-project.github.io/Deskulpt/development/developer-guide/)
has detailed information about contributing code, documentation, tests, and more. We
also provide [developers' API references](https://csci-shu-410-se-project.github.io/Deskulpt/category/references/)
for both the frontend and backend of Deskulpt to help contributors better understand the
codebase.

## Authors

- [Xinyu-Li-123](https://github.com/Xinyu-Li-123)
- [Charlie-XIAO](https://github.com/Charlie-XIAO)
- [ROMEEZHOU](https://github.com/ROMEEZHOU)
- [rz2606](https://github.com/rz2606)
