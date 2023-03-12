use egui_glfw_gl::{
    egui::{self, Pos2, Rect, Color32},
    Painter,
};
use glam::{vec2, Vec2, Vec3};
use glfw::{flush_messages, Context};
use glow::{self, HasContext, ARRAY_BUFFER, FLOAT_VEC2, STATIC_DRAW};
use inferno_engine::{
    engine_draw, primitives::quad::Quad, reload::*, shaders::load_default_shaders, window::*,
};
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

    let mut painter = egui_glfw_gl::Painter::new(window.glfw_handle());
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

    let mut quad = Quad::new(None, window.context());
    let mut new_quad_pos = Vec3::ZERO;

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

            ui.add(egui::Slider::new(&mut new_quad_pos.x, -1.0..=1.0).text("X"));
            ui.add(egui::Slider::new(&mut new_quad_pos.y, -1.0..=1.0).text("Y"));
            ui.add(egui::Slider::new(&mut new_quad_pos.z, -1.0..=1.0).text("Z"));
            if ui.button("Set Position").clicked()
            {
                quad.set_position(new_quad_pos);
            }
        });

        let egui::FullOutput {
            platform_output,
            repaint_after: _,
            textures_delta,
            shapes,
        } = egui_ctx.end_frame();

        quad.render(window.context());

        // Egui
        let clipped_shapes = egui_ctx.tessellate(shapes);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        window.handle.swap_buffers();

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
        }

        for (_, event) in flush_messages(&window.events) {
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
