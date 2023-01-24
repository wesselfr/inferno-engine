use std::sync::mpsc::Receiver;

use bgfx_rs::bgfx::PlatformData;
use glfw::Glfw;
pub use glfw::{Window as WindowHandle, WindowEvent};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct WindowSettings<'a> {
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
    pub mode: glfw::WindowMode<'a>,
}

pub struct Window {
    pub handle: WindowHandle,
    pub events: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
}

pub fn get_platform_data(window: &WindowHandle) -> PlatformData {
    let mut pd = PlatformData::new();

    match window.raw_window_handle() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(data) => {
            pd.nwh = data.window as *mut _;
            pd.ndt = data.display as *mut _;
        }
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(data) => {
            pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
            pd.nwh = data.display;
        }

        #[cfg(target_os = "macos")]
        RawWindowHandle::MacOS(data) => {
            pd.nwh = data.ns_window;
        }
        #[cfg(target_os = "windows")]
        RawWindowHandle::Win32(data) => {
            pd.nwh = data.hwnd;
        }
        _ => panic!("Unsupported Window Manager"),
    }

    return pd;
}

impl Window {
    pub fn init(settings: &WindowSettings) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut handle, events) = glfw
            .create_window(
                settings.width as _,
                settings.height as _,
                settings.title,
                settings.mode,
            )
            .expect("Failed to create GLFW window.");

        handle.set_key_polling(true);

        Window {
            handle,
            events,
            glfw,
        }
    }

    pub fn poll_events(&mut self)
    {
        self.glfw.poll_events();
    }
}
