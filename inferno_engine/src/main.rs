use glam::{vec2, Vec2};
use glfw::{Context, flush_messages};
use glow::{self, HasContext, ARRAY_BUFFER, FLOAT_VEC2, STATIC_DRAW};
use inferno_engine::{engine_draw, line::*, reload::*, shaders::load_default_shaders, window::*};
use shared::State;
use std::time::SystemTime;
use egui_glfw_gl::{Painter, egui::{self, Rect, Pos2}};

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

    //let egui_ctx = egui::Context::default();
    println!("GL VERSION: {:?}", window.context().version());
    
    let mut painter = egui_glfw_gl::Painter::new(&mut window.glfw_handle());
    let egui_ctx = egui::Context::default();
    let native_pixels_per_point = window.handle.get_content_scale().0;

    let mut egui_input_state = egui_glfw_gl::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            egui::vec2(WIDTH as f32, HEIGHT as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let program;
    let vbo;
    let vao;
    unsafe {
        let vertices = [
            vec2(0.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 0.0),
            vec2(1.0, 1.0),
            vec2(1.0, 0.0),
        ];
        let vertices_u8 = std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * std::mem::size_of::<Vec2>(),
        );

        let gl = window.context();

        program = window
            .context()
            .create_program()
            .expect("Cannot create program");

        let shaders = load_default_shaders(program, window.context());

        window.context().link_program(program);
        if !window.context().get_program_link_status(program) {
            panic!("{}", window.context().get_program_info_log(program));
        }

        // VBO
        vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

        // VAO
        vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

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
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);

        egui::Window::new("Egui with GLFW").show(&egui_ctx, |ui| {
            egui::TopBottomPanel::top("Top").show(&egui_ctx, |ui| {
                ui.menu_button("File", |ui| {
                    {
                        let _ = ui.button("test 1");
                    }
                    ui.separator();
                    {
                        let _ = ui.button("test 2");
                    }
                });
            });

            //Image just needs a texture id reference, so we just pass it the texture id that was returned to us
            //when we previously initialized the texture.
            ui.separator();
            ui.label("A simple sine wave plotted onto a GL texture then blitted to an egui managed Image.");
            ui.label(" ");
        });

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        unsafe {
            window.context().use_program(Some(program));
            window.context().bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            window.context().bind_vertex_array(Some(vao));

            window.context().draw_arrays(glow::TRIANGLES, 0, 6);

            // Egui
            let clipped_shapes = egui_ctx.tessellate(shapes);
            painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

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

        for (_, event) in flush_messages(&window.events) {
            match event {
                _ => {
                    egui_glfw_gl::handle_event(event, &mut egui_input_state);
                }
            }
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
