use shared::*;

struct GameState {
    version: u32,
    time_passed: f32,
    test_color: (f32, f32, f32),
}

static mut game_state: GameState = GameState {
    version: 1,
    time_passed: 0.0,
    test_color: (0.0, 0.0, 0.0),
};

#[no_mangle]
pub fn setup(shared: &State) {
    println!("Application version: {}", shared.version);
    shared.finalize();

    let index = shared.load_shader(&vec![ShaderDefinition {
        path: "assets/shaders/compute_shader.comp".to_string(),
        shader_type: ShaderType::Compute,
    }]);
    println!("Shader loaded index: {}", index.unwrap());
}

#[no_mangle]
pub fn update(dt: f32, shared: &mut State) {
    shared.set_clear_color(0x103030ff);

    unsafe {
        game_state.time_passed += dt;
        let time = game_state.time_passed;
        game_state.test_color = (time.sin(), time.cos(), (time * 0.5).sin());
    }
}

#[no_mangle]
pub fn draw(shared: &State) {
    shared.activate_shader(0);
    shared.set_uniform_1_f32(0, "sphere_radius", 3.0);

    let color = unsafe { game_state.test_color };

    shared.set_uniform_3_f32(0, "sphere_color", color.0, color.1, color.2);
    shared.dispatch_compute(512, 512, 1);
}
