use egui_glfw_gl::gl::MAX_HEIGHT;
use glow::{HasContext, NativeTexture};

pub struct Texture {
    width: usize,
    height: usize,
    handle: NativeTexture,
    access: TextureAccess,
}

pub enum TextureAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Texture {
    pub fn new(context: &glow::Context, width: usize, height: usize) -> Texture {
        let handle: NativeTexture;
        unsafe {
            context.active_texture(glow::TEXTURE0);
            handle = context.create_texture().unwrap();
            context.bind_texture(glow::TEXTURE_2D, Some(handle));
            context.texture_parameter_i32(handle, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            context.texture_parameter_i32(handle, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            context.texture_parameter_i32(handle, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            context.texture_parameter_i32(handle, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            context.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA32F as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::FLOAT,
                None,
            );
            // context.tex_storage_2d(
            //     glow::TEXTURE_2D,
            //     0,
            //     glow::RGBA,
            //     width as i32,
            //     height as i32
            // );
        }

        Texture {
            width,
            height,
            handle,
            access: TextureAccess::ReadOnly,
        }
    }

    pub unsafe fn bind(&self, context: &glow::Context) {
        let access = match self.access {
            TextureAccess::ReadOnly => glow::READ_ONLY,
            TextureAccess::WriteOnly => glow::WRITE_ONLY,
            TextureAccess::ReadWrite => glow::READ_WRITE,
        };

        context.bind_texture(glow::TEXTURE_2D, Some(self.handle));
        context.bind_image_texture(0, self.handle, 0, false, 0, access, glow::RGBA32F);
    }

    pub fn resize(&mut self, context: &glow::Context, width: usize, height: usize) {
        unsafe {
            context.active_texture(glow::TEXTURE0);
            self.width = width;
            self.height = height;
            context.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA32F as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::FLOAT,
                None,
            );
        }
    }

    pub fn from_file(path: &str) -> Option<Texture> {
        todo!("Loading texture from path not yet supported.");
    }

    pub fn set_texture_access(&mut self, access: TextureAccess) {
        self.access = access;
    }
}
