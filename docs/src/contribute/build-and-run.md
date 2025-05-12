# Build and Run

## Prerequisites

- Install the system dependencies required by [Tauri v2](https://v2.tauri.app/start/prerequisites/#system-dependencies).
- Install Rust via [rustup](https://www.rust-lang.org/tools/install).
- Install Node.js (LTS version) from its [official website](https://nodejs.org/).
- Install [pnpm](https://pnpm.io/installation). Deskulpt does not support other node package managers.

Make sure you are working on your own fork of the Deskulpt repository. Then install all dependencies from the pnpm workspaces:

```bash
pnpm install
```

## Development

To develop Deskulpt, use the following command:

```bash
pnpm tauri dev
```

The first run might be slow because Rust needs to fetch and compile all dependencies, but subsequent runs should be significantly faster. Once Rust has finished the compilation, the application will be automatically started, and any change you make in the source code will trigger a reload or rebuild.

To debug Deskulpt frontend, you can open the web inspector to debug the Deskulpt frontend in the same way as in a browser. Simply right click the webview and click "Inspect", or use the `Ctrl+Shift+I` (Windows/Linux) or `Cmd+Option+I` (macOS) shortcut. To debug Deskulpt backend, the printouts will appear in the terminal that you ran the command.

Thanks to the [Vite development server](https://vite.dev/guide/cli.html#dev-server), if you are making frequent changes only to the frontend, the hot reloading is very fast without needing to rebuild the app. However if you are making frequent changes to the backend, each time you make a change, a rebuild of the backend is triggered and the whole app will restart. In such cases, you might want to separate the build and run steps to avoid frequent rebuild:

```bash
pnpm tauri build --debug --no-bundle
./target/debug/deskulpt
```

## Build

To make a full release build, run the following command:

```bash
pnpm tauri build
```

Refer to its printout for the locations of the bundled artifacts. These are the final deliverables to the users. You can use them install Deskulpt and run the app as a normal user.

You can also start the app by directly running the executable, so that you can see the printouts in the terminal, but you should keep in mind that this is different from using the bundled artifacts:

```bash
./target/release/deskulpt
```

Note that the web inspector is not enabled in release builds. If you do need to debug the frontend, make a debug build instead:

```bash
pnpm tauri build --debug
```

Again, refer to its printout for the locations of the bundled artifacts, or use the following command to directly run the executable:

```bash
./target/debug/deskulpt
```

If you want only the executable but not the bundled artifacts, you can add the `--no-bundle` flag to the build command.
