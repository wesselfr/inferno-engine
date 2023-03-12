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

pub fn create_default_program(context: &glow::Context) -> Result<glow::NativeProgram, String> {
    unsafe {
        let program = context.create_program();

        // Early return on error.
        program.as_ref()?;

        let program = program.unwrap();
        let shaders = load_default_shaders(program, context);
        context.link_program(program);

        if !context.get_program_link_status(program) {
            return Err(context.get_program_info_log(program));
        }

        for shader in shaders {
            context.detach_shader(program, shader);
            context.delete_shader(shader);
        }

        Ok(program)
    }
}

pub fn load_default_shaders(
    program: glow::NativeProgram,
    context: &glow::Context,
) -> [NativeShader; 2] {
    let shader_version = "#version 410";
    let (vertex_shader_source, fragment_shader_source) = (
        r#"
        in vec2 _pos;
        uniform mat4 _mvp;
        out vec2 vert;
    void main() {
        vert = _pos;
        gl_Position = _mvp * vec4(_pos.x, _pos.y, 1.0, 1.0);
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
