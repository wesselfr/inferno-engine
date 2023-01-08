use bgfx_rs::bgfx::{
    self, ClearFlags, Init, PlatformData, RendererType, ResetFlags, SetViewClearArgs, ResetArgs,
};
use glfw::Window;
use inferno_engine::{engine_draw, reload::*};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use shared::State;
use std::time::SystemTime;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn get_platform_data(window: &Window) -> PlatformData {
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

#[cfg(target_os = "linux")]
fn get_render_type() -> RendererType {
    RendererType::Vulkan
}

#[cfg(not(target_os = "linux"))]
fn get_render_type() -> RendererType {
    RendererType::Count
}

fn main() {
    let mut test = State {
        version: 1,
        draw_fn: engine_draw,
        clear_color: 0x103030ff,
    };

    let mut app: Application;
    app = load_lib();

    let mut last_modified = SystemTime::now();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(
            WIDTH as _,
            HEIGHT as _,
            "cubes.rs bgfx-rs example - ESC to close",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.width = WIDTH as u32;
    init.resolution.height = HEIGHT as u32;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = get_platform_data(&window);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    let mut old_size = (0, 0);
    while !window.should_close() {
        glfw.poll_events();

        // Set clear color
        bgfx::set_view_clear(
            0,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: test.clear_color,
                ..Default::default()
            },
        );

        app.update(&test);

        if should_reload(last_modified) {
            println!("== NEW VERSION FOUND ==");
            app = reload(app);
            println!("== NEW VERSION LOADED ==");
            test.version += 1;
            last_modified = SystemTime::now();
            app.setup(&test);
            app.update(&test);
        }

        let size = window.get_framebuffer_size();

        if old_size != size {
            bgfx::reset(size.0 as _, size.1 as _, ResetArgs::default());
            old_size = size;
        }

        bgfx::set_view_rect(0, 0, 0, size.0 as _, size.1 as _);
        bgfx::touch(0);

        bgfx::frame(false);
    }
    bgfx::shutdown();
}
