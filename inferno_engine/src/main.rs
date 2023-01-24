use bgfx_rs::bgfx::{
    self, ClearFlags, Init, RendererType, ResetArgs, ResetFlags, SetViewClearArgs,
};
use inferno_engine::{engine_draw, reload::*, window::*};
use shared::State;
use std::time::SystemTime;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

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

    let settings = WindowSettings {
        width: WIDTH,
        height: HEIGHT,
        title: "Inferno Engine",
        mode: glfw::WindowMode::Windowed,
    };
    let mut window: Window = Window::init(&settings);

    let mut init = Init::new();

    init.type_r = get_render_type();
    init.resolution.width = WIDTH as u32;
    init.resolution.height = HEIGHT as u32;
    init.resolution.reset = ResetFlags::VSYNC.bits();
    init.platform_data = get_platform_data(&window.handle);

    if !bgfx::init(&init) {
        panic!("failed to init bgfx");
    }

    let mut old_size = (0, 0);
    while !window.handle.should_close() {
        window.poll_events();

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

        let size = window.handle.get_framebuffer_size();

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
