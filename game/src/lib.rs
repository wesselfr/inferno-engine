use shared::State;

#[no_mangle]
pub fn setup(test: &State) {
    println!("Application version: {}", test.version);
    test.finalize();
}

#[no_mangle]
pub fn update(test: &mut State) {
    test.test_string = "Hello World!".to_string();
    test.set_clear_color(0x103030ff);
    test.draw_text(0, 9, &format!("Current Version: {}", test.version));
    test.draw_text(0, 10, "Custom draw function working.");
    test.draw_text(0, 12, "Roguelike");
}
