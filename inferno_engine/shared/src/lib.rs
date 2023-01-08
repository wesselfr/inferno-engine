type FnPtrU32 = fn(u32);

pub struct State {
    pub version: u32,
    pub draw_fn: FnPtrU32,
    pub clear_color: u32,
}

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
    pub fn set_clear_color(&mut self, color: u32){
        self.clear_color = color;
    }
}
