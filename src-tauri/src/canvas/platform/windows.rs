use tauri::Window;
use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            AnimateWindow, DefWindowProcW, GetWindowLongPtrW, RegisterClassW,
            SetWindowLongPtrW, SetWindowPos, AW_BLEND, AW_CENTER, AW_HIDE,
            GWLP_WNDPROC, GWL_EXSTYLE, HWND_BOTTOM, SWP_DRAWFRAME, SWP_HIDEWINDOW,
            SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOMOVE, SWP_NOREDRAW, SWP_NOSIZE,
            WM_KILLFOCUS, WM_MOUSEACTIVATE, WM_SETFOCUS, WNDCLASSA, WS_EX_NOACTIVATE,
        },
    },
};

pub(crate) fn platform_set_window_to_bottom(window: &Window) {
    // // Cast to HWND (specific to Windows platform)
    // let hwnd = window.hwnd().unwrap();
    let hwnd: HWND = HWND(window.hwnd().unwrap().0);
    unsafe {
        SetWindowPos(
            hwnd,
            // this flag set the window to be the bottom-most window
            HWND_BOTTOM,
            0,
            0,
            0,
            0,
            // when calling this function, we want to
            // - not activate the window
            // - not move the window
            // - not resize the window
            SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE,
        )
        .unwrap();
    }
    println!("Window set to bottom (Windows)");
}

extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        // When the window is focused...
        WM_SETFOCUS => {
            println!("Window got focus");
            unsafe {
                // Set the window to the bottom-most window
                SetWindowPos(
                    hwnd,
                    HWND_BOTTOM,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE,
                )
                .unwrap();
                // DefWindowProcW(hwnd, WM_KILLFOCUS, wparam, lparam)
                // DefWindowProcW(hwnd, msg, wparam, lparam)
                LRESULT(0)
            }
        },
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

pub(crate) fn platform_set_window_always_to_bottom(window: &Window) -> Result<()> {
    platform_set_window_to_bottom(window);
    let hwnd = HWND(window.hwnd().unwrap().0);
    unsafe {
        SetWindowLongPtrW(hwnd, GWLP_WNDPROC, window_proc as isize);
        // Get parent window
    }
    Ok(())
}
