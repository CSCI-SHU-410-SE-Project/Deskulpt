# [Feature] Mouse Click Forward

This branch attempts (again) to implement the feature of "forward mouse click on non-widget area" in a platform-specific way in Tauri. The ultimate goal is to support this feature on windows, macos, x11, and wayland.

## Form of Support

This section provides possible forms of support of this feature. The list below is ranked from the most to least favorable.

- Tauri Plugin that contains custom platform-specific code that compiles conditioning on the target platform.

- A PR to Tauri (or Tao) that implements this feature.

- A customized version of Tauri (or Tao) that implements this feature, in case the PR is rejected / takes too long to be accepted in released version.

## Potential solutions

This section lists github issues, api docs, and blogs that implement this feature in other framework / platform / software. We may be able to do platform-specific coding in our app to support this feature.

- [mpv-player/mpv/issues/8938](https://github.com/mpv-player/mpv/issues/8938)

  This issue points to a few useful solutions of this problem.

- [glfw/glfw/issues/1236](https://github.com/glfw/glfw/issues/1236)

  Seems like GLFW has done this already. Maybe their solution could be used in our app.

- [Electron `win.setIgnoreMouseEvents(ignore[, options])`](https://www.electronjs.org/docs/latest/api/browser-window#winsetignoremouseeventsignore-options)

  Electron supports this feature only for Windows and MacOS.
