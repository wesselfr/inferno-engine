use std::num::NonZeroU32;

use glow::*;

pub struct Shader<'a> {
    shader_type: u32,
    shader_source: &'a str,
}

pub fn create_shader(
    context: &glow::Context,
    source: &Shader,
    shader_version: &str,
) -> Result<NativeShader, String> {
    let shader;
    unsafe {
        shader = context
            .create_shader(source.shader_type)
            .expect("Cannot create shader..");

        context.shader_source(
            shader,
            &format!("{}\n{}", shader_version, source.shader_source),
        );

        context.compile_shader(shader);
        if !context.get_shader_compile_status(shader) {
            return Err(context.get_shader_info_log(shader));
        }
    }
    Ok(shader)
}

pub fn load_default_shaders(
    program: glow::NativeProgram,
    context: &glow::Context,
) -> [NativeShader; 2] {
    let shader_version = "#version 410";
    let (vertex_shader_source, fragment_shader_source) = (
        r#"const vec2 verts[4] = vec2[4](
        vec2(0.0f, 1.0f),
        vec2(0.0f, 0.0f),
        vec2(1.0f, 0.0f),
        vec2(1.0f, 1.0f)
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

    let mut shaders: [NativeShader; 2] = [glow::NativeShader(NonZeroU32::new(1).unwrap()); 2];

    unsafe {
        for (index, source) in shader_sources.iter().enumerate() {
            let shader = create_shader(
                context,
                &Shader {
                    shader_type: source.0,
                    shader_source: source.1,
                },
                shader_version,
            )
            .expect("Error while creating default shader.");

            context.attach_shader(program, shader);
            shaders[index] = shader;
        }
    }
    shaders
}
