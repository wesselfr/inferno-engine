type FnPtrU32 = fn(u32);

// GRAPHICS
type FnPtrLoadShader = fn(&Vec<ShaderDefinition>) -> Option<u32>;
type FnPtrActivateShader = fn(u32);
type FnPtrUniform1F32 = fn(u32, &str, f32);
type FnPtrUniform2F32 = fn(u32, &str, f32, f32);
type FnPtrUniform3F32 = fn(u32, &str, f32, f32, f32);
type FnPtrDispatchCompute = fn(u32, u32, u32);

pub struct State {
    pub version: u32,
    pub draw_fn: FnPtrU32,
    pub shader_load_fn: FnPtrLoadShader,
    pub shader_activate_fn: FnPtrActivateShader,
    pub shader_uniform_1_f32: FnPtrUniform1F32,
    pub shader_uniform_2_f32: FnPtrUniform2F32,
    pub shader_uniform_3_f32: FnPtrUniform3F32,
    pub shader_dispatch_compute: FnPtrDispatchCompute,
    pub clear_color: u32,
}

pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

pub struct ShaderDefinition {
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

    pub fn activate_shader(&self, shader: Shader) {
        (self.shader_activate_fn)(shader);
    }

    pub fn set_uniform_1_f32(&self, shader: Shader, field: &str, x: f32) {
        (self.shader_uniform_1_f32)(shader, field, x);
    }
    pub fn set_uniform_2_f32(&self, shader: Shader, field: &str, x: f32, y: f32) {
        (self.shader_uniform_2_f32)(shader, field, x, y);
    }
    pub fn set_uniform_3_f32(&self, shader: Shader, field: &str, x: f32, y: f32, z: f32) {
        (self.shader_uniform_3_f32)(shader, field, x, y, z);
    }

    pub fn dispatch_compute(&self, group_x: u32, group_y: u32, group_z: u32) {
        (self.shader_dispatch_compute)(group_x, group_y, group_z);
    }
}
