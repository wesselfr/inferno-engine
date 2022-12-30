type FnPtrU32 = fn(u32);

pub struct State {
    pub version: u32,
    pub draw_fn: FnPtrU32,
}

impl State {
    pub fn finalize(&self) {
        println!("DONE!");
    }
    pub fn get_handle(&self) -> u32 {
        self.version * 3
    }
    pub fn draw(&self, handle: u32) {
        (self.draw_fn)(handle);
    }
}
