use shared::*;

struct GameState {
    time_passed: f32,
    test_color: (f32, f32, f32),
}

static mut GAME_STATE: GameState = GameState {
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
        GAME_STATE.time_passed += dt;
        let time = GAME_STATE.time_passed;
        GAME_STATE.test_color = (time.sin(), time.cos(), (time * 0.5).sin());
    }
}

#[no_mangle]
pub fn draw(shared: &State) {
    shared.activate_shader(0);
    shared.set_uniform_1_f32(0, "voxel_size", unsafe {
        (3.0 + GAME_STATE.time_passed.sin() * 3.0).max(0.5)
    });

    let color = unsafe { GAME_STATE.test_color };

    shared.set_uniform_3_f32(0, "voxel_color", color.0, color.1, color.2);
    shared.dispatch_compute(512, 512, 1);
}
