use std::sync::mpsc::Receiver;

use glfw::Glfw;
pub use glfw::{Window as WindowHandle, WindowEvent};
use glow;
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

impl Window {
    pub fn init(settings: &WindowSettings) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));

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

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}
