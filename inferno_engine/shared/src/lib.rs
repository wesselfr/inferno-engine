type FnPtrU32 = fn(u32);
type FnPtrLoadShader = fn(&Vec<ShaderDefinition>) -> Option<u32>;

pub struct State {
    pub version: u32,
    pub test_string: String,
    pub draw_fn: FnPtrU32,
    pub shader_load_fn: FnPtrLoadShader,
    pub clear_color: u32,
}

pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

pub struct ShaderDefinition
{
    pub path: String,
    pub shader_type: ShaderType,
}

pub type Shader = u32;

impl State {
    pub fn finalize(&self) {
        println!("LIB ACTIVE!");
    }
    pub fn get_handle(&self) -> u32 {
        self.version * 3
    }
    pub fn draw(&self, handle: u32) {
        (self.draw_fn)(handle);
    }
    pub fn set_clear_color(&mut self, color: u32) {
        self.clear_color = color;
    }

    // Graphics API

    // Load shaders from path and create a program.
    // Returns None when running into issues.
    pub fn load_shader(&self, shader_definitions: &Vec<ShaderDefinition>) -> Option<Shader> {
        (self.shader_load_fn)(shader_definitions)
    }

    pub fn activate_shader(shader: Shader) {}

    pub fn set_uniform_1_f32(shader: Shader, x: f32) {}
}
