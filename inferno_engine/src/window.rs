use glam::Vec4;
use glfw::Glfw;
pub use glfw::{Window as WindowHandle, WindowEvent};
use glow::{self, Context, HasContext};
use std::sync::mpsc::Receiver;

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
    gl: Context,
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
        handle.set_cursor_pos_polling(true);
        handle.set_mouse_button_polling(true);

        // TODO: Add some error handeling incase the glContext creation fails.
        let gl = unsafe {
            glow::Context::from_loader_function(|s| handle.get_proc_address(s) as *const _)
        };

        Window {
            handle,
            events,
            glfw,
            gl,
        }
    }

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    pub fn clear(&self, color: Vec4) {
        unsafe {
            self.gl.clear_color(color.x, color.y, color.z, color.w);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn context(&self) -> &Context {
        &self.gl
    }

    pub fn glfw_handle(&mut self) -> &mut WindowHandle
    {
        &mut self.handle
    }
}
