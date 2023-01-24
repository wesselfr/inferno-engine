use bgfx_rs::bgfx::{
    self, ClearFlags, Init, RendererType, ResetArgs, ResetFlags, SetViewClearArgs, DbgTextClearArgs, DebugFlags,
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

fn draw_text(x: u16, y: u16, text: &str)
{
    bgfx::dbg_text(x, y, 0x3f, text);
}

fn main() {
    let mut test = State {
        version: 1,
        test_string: "Hello World".to_string(),
        draw_fn: engine_draw,
        text_draw_fn: draw_text,
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

    bgfx::set_debug(DebugFlags::TEXT.bits());

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
        bgfx::dbg_text_clear(DbgTextClearArgs::default());
        
        app.update(&test);
        
        bgfx::dbg_text(0, 1, 0x0f, "Color can be changed with ANSI \x1b[9;me\x1b[10;ms\x1b[11;mc\x1b[12;ma\x1b[13;mp\x1b[14;me\x1b[0m code too.");
        bgfx::dbg_text(80, 1, 0x0f, "\x1b[;0m    \x1b[;1m    \x1b[; 2m    \x1b[; 3m    \x1b[; 4m    \x1b[; 5m    \x1b[; 6m    \x1b[; 7m    \x1b[0m");
        bgfx::dbg_text(80, 2, 0x0f, "\x1b[;8m    \x1b[;9m    \x1b[;10m    \x1b[;11m    \x1b[;12m    \x1b[;13m    \x1b[;14m    \x1b[;15m    \x1b[0m");
        bgfx::dbg_text(
            0,
            4,
            0x3f,
            "Description: Initialization and debug text with bgfx-rs Rust API.",
        );
        bgfx::dbg_text(0, 5, 0x3f, &test.test_string);

        bgfx::frame(false);
    }
    bgfx::shutdown();
}
