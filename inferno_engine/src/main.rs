use glfw::Context;
use glow::{self, HasContext};
use inferno_engine::{engine_draw, reload::*, window::*};
use shared::State;
use std::time::SystemTime;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut test = State {
        version: 1,
        test_string: "Hello World".to_string(),
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

    println!("GL VERSION: {:?}", window.context().version());
    let shader_version = "#version 410";

    unsafe {
        let vertex_array = window.context()
            .create_vertex_array()
            .expect("Cannot create vertex array");
        window.context().bind_vertex_array(Some(vertex_array));

        let program = window.context().create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
        vec2(0.5f, 1.0f),
        vec2(0.0f, 0.0f),
        vec2(1.0f, 0.0f)
        );
        out vec2 vert;
        void main() {
            vert = verts[gl_VertexID];
            gl_Position = vec4(vert - 0.5, 0.0, 1.0);
        }"#,
            r#"precision mediump float;
        in vec2 vert;
        out vec4 color;
        void main() {
            color = vec4(vert, 0.5, 1.0);
        }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = window.context()
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            window.context().shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            window.context().compile_shader(shader);
            if !window.context().get_shader_compile_status(shader) {
                panic!("{}", window.context().get_shader_info_log(shader));
            }
            window.context().attach_shader(program, shader);
            shaders.push(shader);
        }

        window.context().link_program(program);
        if !window.context().get_program_link_status(program) {
            panic!("{}", window.context().get_program_info_log(program));
        }

        for shader in shaders {
            window.context().detach_shader(program, shader);
            window.context().delete_shader(shader);
        }

        window.context().use_program(Some(program));
        window.context().clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let mut old_size = (0, 0);
    while !window.handle.should_close() {
        window.poll_events();

        // Set clear color
        window.clear(u32_to_vec4(test.clear_color));

        unsafe {
            window.context().draw_arrays(glow::TRIANGLES, 0, 3);
            window.handle.swap_buffers();
        }

        // Reloading
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
            old_size = size;
        }
    }
}

fn u32_to_vec4(val: u32) -> glam::Vec4 {
    let raw: [u8; 4] = val.to_be_bytes();
    glam::vec4(
        raw[0] as f32 / 255.0,
        raw[1] as f32 / 255.0,
        raw[2] as f32 / 255.0,
        raw[3] as f32 / 255.0,
    )
}
