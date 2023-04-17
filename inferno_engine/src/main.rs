use egui_glfw_gl::egui::{self, Pos2, Rect};
use glam::Vec3;
use glfw::{flush_messages, Context};
use glow::{self, HasContext};
use inferno_engine::{
    engine_draw, primitives::quad::Quad, reload::*, shaders::*, texture::*, window::*,
};
use shared::{ShaderDefinition, State};
use std::time::{Instant, SystemTime};

const DEGREES_TO_RADIANS: f32 = 0.01745329;

// Required for now. Needed for the bridge between game and engine.
// TODO: Look into ways to get rid of these globals.
static mut WINDOW: Option<Window> = None;
static mut LOADED_SHADERS: Vec<glow::NativeProgram> = Vec::new();
static mut LOADED_TEXTURES: Vec<glow::NativeTexture> = Vec::new();

fn api_load_shader(shader_definitions: &Vec<ShaderDefinition>) -> Option<u32> {
    let index;
    unsafe {
        let mut shaders: Vec<Shader> = Vec::new();

        for defintion in shader_definitions {
            let shader_type = match defintion.shader_type {
                shared::ShaderType::Vertex => glow::VERTEX_SHADER,
                shared::ShaderType::Fragment => glow::FRAGMENT_SHADER,
                shared::ShaderType::Compute => glow::COMPUTE_SHADER,
            };
            shaders.push(
                load_shader(&defintion.path, shader_type).expect("Error while loading shader."),
            );
        }

        let program = create_shader_program(
            WINDOW.as_ref()?.context(),
            vec![
                load_shader("assets/shaders/default.vert", glow::VERTEX_SHADER)
                    .expect("Error loading vertex shader"),
                load_shader("assets/shaders/default.frag", glow::FRAGMENT_SHADER)
                    .expect("Error loading vertex shader"),
            ],
        )
        .unwrap();
        LOADED_SHADERS.push(program);
        index = LOADED_SHADERS.len() as u32 - 1;
    }

    Some(index)
}

fn api_activate_shader(shader: u32) {
    unsafe {
        let shader = LOADED_SHADERS[shader as usize];
        let ctx = WINDOW.as_ref().unwrap().context();
        ctx.use_program(Some(shader));
    }
}

fn api_set_uniform_1_f32(shader: u32, field: &str, x: f32) {
    unsafe {
        let shader = LOADED_SHADERS[shader as usize];
        let ctx = WINDOW.as_ref().unwrap().context();
        ctx.uniform_1_f32(ctx.get_uniform_location(shader, field).as_ref(), x);
    }
}

fn api_set_uniform_2_f32(shader: u32, field: &str, x: f32, y: f32) {
    unsafe {
        let shader = LOADED_SHADERS[shader as usize];
        let ctx = WINDOW.as_ref().unwrap().context();
        ctx.uniform_2_f32(ctx.get_uniform_location(shader, field).as_ref(), x, y);
    }
}

fn api_set_uniform_3_f32(shader: u32, field: &str, x: f32, y: f32, z: f32) {
    unsafe {
        let shader = LOADED_SHADERS[shader as usize];
        let ctx = WINDOW.as_ref().unwrap().context();
        ctx.uniform_3_f32(ctx.get_uniform_location(shader, field).as_ref(), x, y, z);
    }
}

// Extra API calls

fn api_dispatch_compute(group_x: u32, group_y: u32, group_z: u32) {
    unsafe {
        let ctx = WINDOW.as_ref().unwrap().context();
        ctx.memory_barrier(glow::SHADER_STORAGE_BARRIER_BIT);
        ctx.dispatch_compute(group_x, group_y, group_z);
    }
}

fn setup_api_state() -> State {
    State {
        version: 1,
        draw_fn: engine_draw,
        shader_load_fn: api_load_shader,
        shader_activate_fn: api_activate_shader,
        shader_uniform_1_f32: api_set_uniform_1_f32,
        shader_uniform_2_f32: api_set_uniform_2_f32,
        shader_uniform_3_f32: api_set_uniform_3_f32,
        shader_dispatch_compute: api_dispatch_compute,
        clear_color: 0x103030ff,
    }
}

fn main() {
    let mut shared_state = setup_api_state();

    let mut app: Application;
    app = load_lib();

    let mut last_modified = SystemTime::now();

    unsafe {
        WINDOW = Some(Window::init(None));
    }

    let window_handle;
    unsafe {
        window_handle = WINDOW.as_mut().expect("Window was not set.");
    }

    let ctx;
    unsafe {
        ctx = WINDOW
            .as_ref()
            .expect("Window was not initialized.")
            .context();
    }
    println!("GL VERSION: {:?}", ctx.version());

    let mut painter;
    unsafe {
        painter = egui_glfw_gl::Painter::new(
            WINDOW
                .as_mut()
                .expect("Window not initalized.")
                .glfw_handle(),
        );
    }
    let egui_ctx = egui::Context::default();
    let native_pixels_per_point = 1.0;

    let mut egui_input_state = egui_glfw_gl::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::new(0f32, 0f32), {
            let window_size: (usize, usize) = window_handle.get_size();
            egui::vec2(window_size.0 as f32, window_size.1 as f32) / native_pixels_per_point
        })),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let quad_shader = create_shader_program(
        ctx,
        vec![
            load_shader("assets/shaders/default.vert", glow::VERTEX_SHADER)
                .expect("Error loading vertex shader"),
            load_shader("assets/shaders/default.frag", glow::FRAGMENT_SHADER)
                .expect("Error loading vertex shader"),
        ],
    )
    .unwrap();

    let mut quad_texture = Texture::new(ctx, 512, 512);
    quad_texture.set_texture_access(TextureAccess::WriteOnly);

    let mut quad = Quad::new(Some(quad_shader), ctx);
    let mut new_quad_pos = Vec3::ZERO;

    let mut ray_shader = create_shader_program(
        ctx,
        vec![
            load_shader("assets/shaders/compute_shader.comp", glow::COMPUTE_SHADER)
                .expect("Error while loading compute shader"),
        ],
    )
    .unwrap();

    unsafe {
        LOADED_SHADERS.push(ray_shader);
    }

    let mut old_size = (0, 0);
    let mut dt = 0.0;
    let mut frame_start_time = Instant::now();

    app.setup(&shared_state);

    while !window_handle.handle.should_close() {
        frame_start_time = Instant::now();

        window_handle.poll_events();
        // Set clear color
        window_handle.clear(u32_to_vec4(shared_state.clear_color));
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

                let shader_source = load_shader("assets/shaders/compute_shader.comp", glow::COMPUTE_SHADER).expect("Error while reading shader source.");
                let shader = create_shader_program(
                    ctx,
                    vec![
                        shader_source,
                    ],
                );

                match shader {
                    Ok(shader) => {ray_shader = shader;
                        unsafe{
                            LOADED_SHADERS[0] = shader;
                        }
                        println!("Shader reloaded!");
                    },
                    Err(e) => println!("Error while reloading shader: {:?}", e),
                }
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

            let clear_color = u32_to_vec4(shared_state.clear_color);

            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "clear_color").as_ref(),
                clear_color.x,
                clear_color.y,
                clear_color.z,
            );

            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "voxel_center")
                    .as_ref(),
                0.0,
                0.0,
                10.0,
            );
            ctx.uniform_1_f32(
                ctx.get_uniform_location(ray_shader, "voxel_size").as_ref(),
                3.0,
            );
            ctx.uniform_3_f32(
                ctx.get_uniform_location(ray_shader, "voxel_color").as_ref(),
                0.0,
                1.0,
                1.0,
            );

            // Bind screen texture.
            ctx.active_texture(glow::TEXTURE0);
            quad_texture.set_texture_access(TextureAccess::ReadWrite);
            quad_texture.bind(ctx);

            app.draw(&shared_state);
        }

        unsafe {
            ctx.use_program(Some(quad_shader));
            quad_texture.set_texture_access(TextureAccess::ReadOnly);
            quad_texture.bind(ctx);
        }

        quad.render(ctx);

        // Egui
        let clipped_shapes = egui_ctx.tessellate(shapes);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        window_handle.glfw_handle().swap_buffers();

        // Reloading
        if should_reload(last_modified) {
            println!("== NEW VERSION FOUND ==");
            app = reload(app);
            println!("== NEW VERSION LOADED ==");
            shared_state.version += 1;
            last_modified = SystemTime::now();
            app.setup(&shared_state);
        }

        app.update(dt, &shared_state);

        let size = window_handle.glfw_handle().get_framebuffer_size();

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
                quad_texture.bind(ctx);
                quad_texture.resize(ctx, size.0 as usize, size.1 as usize);
            }
        }

        for (_, event) in flush_messages(&window_handle.events) {
            {
                egui_glfw_gl::handle_event(event, &mut egui_input_state);
            }
        }

        dt = frame_start_time.elapsed().as_secs_f32();
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
