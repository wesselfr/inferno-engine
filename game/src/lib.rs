use shared::State;

#[no_mangle]
pub fn get_message() -> &'static str {
    "Hello World!!"
}

#[no_mangle]
pub fn setup(test: &State)
{
    println!("Application version: {}", test.version);
    test.finalize();
}

#[no_mangle]
pub fn update(test: &State)
{
    println!("Update");
    let handle = test.get_handle();
    test.draw(handle);
}