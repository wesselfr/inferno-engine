use glam::{Mat4, Vec3, Vec4};
use glow::*;

struct RenderData {
    vbo: Option<NativeBuffer>,
    vao: Option<NativeVertexArray>,
    shader_program: Option<NativeProgram>,
    mvp: Mat4,
}

pub struct Line {
    render_data: RenderData,
    points: Vec<Vec3>,
    color: Vec4,
}

impl Line {
    pub fn new(shader_program: Option<NativeProgram>, context: &Context) -> Self {
        let mut render_data = RenderData {
            vbo: None,
            vao: None,
            shader_program,
            mvp: Mat4::IDENTITY,
        };

        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)];

        let vbo = unsafe {
            context
                .create_buffer()
                .expect("Error while creating buffer.")
        };
        unsafe {
            let data: &[u8] = std::slice::from_raw_parts(
                points.as_ptr() as *const u8,
                points.len() * std::mem::size_of::<f32>(),
            );
            context.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            context.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STATIC_DRAW);
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
            context.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                3 * std::mem::size_of::<f32>() as i32,
                0,
            );
        }

        render_data.vbo = Some(vbo);
        render_data.vao = Some(vao);

        let color = Vec4::new(1.0, 1.0, 1.0, 1.0);

        Line {
            render_data,
            points,
            color,
        }
    }
}
