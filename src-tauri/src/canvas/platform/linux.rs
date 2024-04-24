use gdk::glib::translate::ToGlibPtr;
use tauri::window::Window;
use x11::xlib::{Display, XLowerWindow};
// use gtk::prelude::GtkWindowExt;
use gdk::{prelude::DisplayExtManual, Backend};
use gdk_x11_sys::{gdk_x11_display_get_xdisplay, GdkX11Display};
use gdkx11::{X11Display, X11Window};
use gtk::prelude::{Cast, WidgetExt};
use gtk::ApplicationWindow;

pub(crate) fn platform_set_window_to_bottom(window: &Window) {
    // Cast to XWindow (specific to X11/Linux)
    // let x_window = window.xlib_window().unwrap() as XWindow;
    // window.gtk
    // let display = window.xlib_display().unwrap() as *mut Display;
    // unsafe {
    //     XLowerWindow(display, x_window);
    // }
    // println!("Window set to bottom (Linux)");
    // window.gtk_window();

    // Get X11 window
    let gtk_window = window.gtk_window().unwrap();
    let gdk_window = gtk_window.window().unwrap();
    let gdk_display = gdk_window.display();
    let backend = gdk_display.backend();
    match backend {
        Backend::X11 => {
            // get xid
            let x11_window = gdk_window.downcast::<X11Window>().unwrap();
            let xid = x11_window.xid();

            // get xdisplay
            let x11_display = gdk_display.downcast::<X11Display>().unwrap();
            let xdisplay_ptr = x11_display.to_glib_none().0;

            unsafe {
                // XLowerWindow(xdisplay, xid);
                let xdisplay = gdk_x11_display_get_xdisplay(xdisplay_ptr);
                XLowerWindow(xdisplay, xid);
            }
            println!("Backend: {:?}", backend);
            println!("Window xid: {}", xid);
            println!("Window set to bottom (Linux)");
        },
        _ => {
            panic!("Unsupported backend")
        },
    }
}
