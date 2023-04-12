use egui_glfw_gl::egui::{self, Pos2, Rect};
use glam::Vec3;
use glfw::{flush_messages, Context};
use glow::{self, HasContext};
use inferno_engine::{
    engine_draw, primitives::quad::Quad, reload::*, shaders::*, texture::*, window::*,
};
use shared::{State, ShaderDefinition};
use std::{time::SystemTime, sync::Mutex};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const DEGREES_TO_RADIANS: f32 = 0.01745329;

struct Sphere {
    center: Vec3,
    radius: f32,
    color: f32,
}

// Required for now. Needed for the bridge between game and engine.
// TODO: Look into ways to get rid of these globals.
static mut window: Option<Window> = None;
static mut loaded_shaders: Vec<glow::NativeProgram> = Vec::new();

fn api_load_shader(shader_definitions: &Vec<ShaderDefinition>)-> Option<u32>
{
    let index;
    unsafe{

    let mut shaders: Vec<Shader> = Vec::new();

    for defintion in shader_definitions
    {
        let shader_type = match defintion.shader_type {
            shared::ShaderType::Vertex => glow::VERTEX_SHADER,
            shared::ShaderType::Fragment => glow::FRAGMENT_SHADER,
            shared::ShaderType::Compute => glow::COMPUTE_SHADER,
        };
        shaders.push(load_shader(&defintion.path, shader_type).expect("Error while loading shader."));
    }
    
    let program = create_shader_program(
        &window.as_ref()?.context(),
        vec![
            load_shader("assets/shaders/default.vert", glow::VERTEX_SHADER)
                .expect("Error loading vertex shader"),
            load_shader("assets/shaders/default.frag", glow::FRAGMENT_SHADER)
                .expect("Error loading vertex shader"),
        ],
    )
    .unwrap();
        loaded_shaders.push(program);
        index = loaded_shaders.len() as u32 - 1;
    }

    Some(index)
}

fn main() {
    let mut test = State {
        version: 1,
        test_string: "Hello World".to_string(),
        draw_fn: engine_draw,
        shader_load_fn: api_load_shader,
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

    unsafe{
        window = Some(Window::init(&settings));
    }

    let ctx;
    unsafe{
        ctx = window.as_ref().expect("Window was not initialized.").context();
    }
    println!("GL VERSION: {:?}", ctx.version());

    let mut painter;
    unsafe{
        painter = egui_glfw_gl::Painter::new(window.as_mut().expect("Window not initalized.").glfw_handle());
    }
    let egui_ctx = egui::Context::default();
    let native_pixels_per_point = 1.0;

    let mut egui_input_state = egui_glfw_gl::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            egui::vec2(WIDTH as f32, HEIGHT as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let quad_shader = create_shader_program(
        &ctx,
        vec![
            load_shader("assets/shaders/default.vert", glow::VERTEX_SHADER)
                .expect("Error loading vertex shader"),
            load_shader("assets/shaders/default.frag", glow::FRAGMENT_SHADER)
                .expect("Error loading vertex shader"),
        ],
    )
    .unwrap();

    let mut quad_texture = Texture::new(&ctx, 512, 512);
    quad_texture.set_texture_access(TextureAccess::WriteOnly);

    let mut quad = Quad::new(Some(quad_shader), &ctx);
    let mut new_quad_pos = Vec3::ZERO;

    let mut ray_shader = create_shader_program(
        &ctx,
        vec![
            load_shader("assets/shaders/compute_shader.comp", glow::COMPUTE_SHADER)
                .expect("Error while loading compute shader"),
        ],
    )
    .unwrap();

    let mut old_size = (0, 0);

    let window_handle;
    unsafe{
        window_handle = window.as_mut().expect("Window was not set.");
    }

    app.setup(&test);

    while !window_handle.handle.should_close() {
        window_handle.poll_events();
        // Set clear color
        window_handle.clear(u32_to_vec4(test.clear_color));
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

            ui.add(egui::Slider::new(&mut new_quad_pos.x, -10.0..=10.0).text("X"));
            ui.add(egui::Slider::new(&mut new_quad_pos.y, -10.0..=10.0).text("Y"));
            ui.add(egui::Slider::new(&mut new_quad_pos.z, -10.0..=10.0).text("Z"));

            if ui.button("Reload shader").clicked()
            {
                ray_shader = create_shader_program(
                    &ctx,
                    vec![
                        load_shader("assets/shaders/compute_shader.comp", glow::COMPUTE_SHADER)
                            .expect("Error while loading compute shader"),
                    ],
                )
                .unwrap();
            }
        });

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        // Compute shader
        unsafe {
            //glActiveTexture(GL_TEXTURE0);
            ctx.memory_barrier(glow::SHADER_STORAGE_BARRIER_BIT);
            ctx.use_program(Some(ray_shader));

            // Setup unforms
            // TODO: Make use of an uniform buffer object to pass all the data at once.
            let ctx = ctx;

            ctx.uniform_1_f32(
                ctx.get_uniform_location(ray_shader, "u_width").as_ref(),
                old_size.0 as f32,
            );
            ctx.uniform_1_f32(
                ctx.get_uniform_location(ray_shader, "u_height").as_ref(),
                old_size.1 as f32,
            );

            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "camera_position")
                    .as_ref(),
                new_quad_pos.x,
                new_quad_pos.y,
                new_quad_pos.z,
            );
            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "camera_up").as_ref(),
                0.0,
                1.0,
                0.0,
            );
            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "camera_forward")
                    .as_ref(),
                0.0,
                0.0,
                1.0,
            );
            ctx.uniform_1_f32(
                ctx.get_uniform_location(ray_shader, "fov_y").as_ref(),
                60.0 * DEGREES_TO_RADIANS,
            );

            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "sphere_center")
                    .as_ref(),
                0.0,
                0.0,
                10.0,
            );
            ctx.uniform_1_f32(
                ctx.get_uniform_location(ray_shader, "sphere_radius")
                    .as_ref(),
                3.0,
            );
            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "sphere_color")
                    .as_ref(),
                0.0,
                1.0,
                1.0,
            );

            ctx.active_texture(glow::TEXTURE0);
            quad_texture.set_texture_access(TextureAccess::ReadWrite);
            //glBindTexture(GL_TEXTURE_2D, tex_output);
            quad_texture.bind(&ctx);

            ctx.dispatch_compute(old_size.0 as u32, old_size.1 as u32, 1);

            ctx.memory_barrier(glow::SHADER_STORAGE_BARRIER_BIT);
        }

        unsafe {
            ctx.use_program(Some(quad_shader));
            quad_texture.set_texture_access(TextureAccess::ReadOnly);
            quad_texture.bind(&ctx);
        }

        quad.render(&ctx);

        // Egui
        let clipped_shapes = egui_ctx.tessellate(shapes);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        unsafe{
            window_handle.glfw_handle().swap_buffers();
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

        app.update(&test);

        let size;
        unsafe{
             size = window_handle.glfw_handle().get_framebuffer_size();
        }

        if old_size != size {
            old_size = size;

            // Update egui sizes
            painter.set_size(size.0 as u32, size.1 as u32);
            egui_input_state = egui_glfw_gl::EguiInputState::new(egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::new(0f32, 0f32),
                    egui::vec2(size.0 as f32, size.1 as f32) / native_pixels_per_point,
                )),
                pixels_per_point: Some(native_pixels_per_point),
                ..Default::default()
            });

            // Update texture
            unsafe {
                ctx.use_program(Some(quad_shader));
                quad_texture.set_texture_access(TextureAccess::ReadWrite);
                quad_texture.bind(&ctx);
                quad_texture.resize(&ctx, size.0 as usize, size.1 as usize);
            }
        }

        for (_, event) in flush_messages(&window_handle.events) {
            {
                egui_glfw_gl::handle_event(event, &mut egui_input_state);
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
