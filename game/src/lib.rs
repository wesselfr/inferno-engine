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
}
