use tauri::Window;
use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::{
            DefWindowProcA, RegisterClassA, SetWindowLongPtrA, GWLP_WNDPROC,
            WM_SETFOCUS, WNDCLASSA,
        },
    },
};

#[cfg(target_os = "windows")]
pub(crate) fn platform_set_window_to_bottom(window: &Window) {
    use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
        },
    };

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
    println!("Window set to always on bottom (Windows)");
}

extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_SETFOCUS => {
            println!("Window got focus");
            unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) }
        },
        _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
}

pub(crate) fn platform_set_window_always_to_bottom(window: &Window) -> Result<()> {
    let hwnd = HWND(window.hwnd().unwrap().0);
    unsafe {
        let original_proc = SetWindowLongPtrA(hwnd, GWLP_WNDPROC, window_proc as isize);
    }
    Ok(())
}
