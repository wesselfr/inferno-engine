use shared::*;


struct GameState {
    version: u32,
}

static mut game_state: GameState = GameState { version: 1 };

#[no_mangle]
pub fn setup(test: &State) {
    println!("Application version: {}", test.version);
    test.finalize();

    let index = test.load_shader(&vec![ShaderDefinition{ path: "assets/shaders/compute_shader.comp".to_string(), shader_type: ShaderType::Compute }]);
    println!("Shader loaded index: {}", index.unwrap());
}

#[no_mangle]
pub fn update(test: &mut State) {
    test.set_clear_color(0x103030ff);

    unsafe {
        game_state.version += 1;
        //println!("Frames: {}", game_state.version);
    }

}

#[no_mangle]
pub fn draw(test: &State) {}
