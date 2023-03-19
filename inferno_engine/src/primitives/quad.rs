use glam::{vec2, Mat4, Vec2, Vec3, Vec4};
use glow::*;

use crate::shaders::create_default_program;

struct RenderData {
    vbo: Option<NativeBuffer>,
    vao: Option<NativeVertexArray>,
    shader_program: Option<NativeProgram>,
    mvp: Mat4,
}

pub struct Quad {
    render_data: RenderData,
    vertices: [Vec2; 6],
    color: Vec4,
}

impl Quad {
    pub fn new(shader_program: Option<NativeProgram>, context: &Context) -> Self {
        let mut render_data = RenderData {
            vbo: None,
            vao: None,
            shader_program,
            mvp: Mat4::IDENTITY,
        };

        if shader_program.is_none() {
            render_data.shader_program = Some(
                create_default_program(context)
                    .expect("Error while creating default shader program."),
            );
        }

        let vertices = [
            vec2(0.0, 0.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 0.0),
            vec2(1.0, 1.0),
            vec2(1.0, 0.0),
        ];

        let vbo = unsafe {
            context
                .create_buffer()
                .expect("Error while creating buffer.")
        };

        let vertices_u8 = unsafe {
            std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * std::mem::size_of::<Vec2>(),
            )
        };

        unsafe {
            context.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            context.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);
        }

        // VAO
        let vao = unsafe {
            context
                .create_vertex_array()
                .expect("Error while creating vertex array.")
        };
        unsafe {
            context.bind_vertex_array(Some(vao));
            context.enable_vertex_attrib_array(0);
            context.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);
        }

        render_data.vbo = Some(vbo);
        render_data.vao = Some(vao);

        let color = Vec4::new(1.0, 1.0, 1.0, 1.0);

        Quad {
            render_data,
            vertices,
            color,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        let (scale, rotation, _) = self.render_data.mvp.to_scale_rotation_translation();
        let translation = position;
        self.render_data.mvp = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn get_position(&self) -> Vec3 {
        let (_, _, translation) = self.render_data.mvp.to_scale_rotation_translation();
        translation
    }

    pub fn render(&mut self, context: &Context) {
        // TODO: Given that the majority of function calls depends on the render_data, it might be worthwhile creating a trait for rendering.
        unsafe {
            context.use_program(self.render_data.shader_program);
            context.bind_buffer(glow::ARRAY_BUFFER, self.render_data.vbo);
            context.bind_vertex_array(self.render_data.vao);

            let (scale, rotation, translation) =
                self.render_data.mvp.to_scale_rotation_translation();
            self.render_data.mvp =
                Mat4::from_scale_rotation_translation(scale, rotation, translation);

            let uniform_location =
                context.get_uniform_location(self.render_data.shader_program.unwrap(), "_mvp");
            context.uniform_matrix_4_f32_slice(
                Some(&uniform_location.unwrap()),
                false,
                &self.render_data.mvp.to_cols_array(),
            );

            context.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
