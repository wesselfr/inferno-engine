extern crate libloading;

use libloading::Library;

const LIB_PATH: &'static str = "../app/target/debug/app.dll";

struct Application(Library);
impl Application {
    fn get_message(&self) -> &'static str {
        unsafe {
            let f = self
                .0
                .get::<fn() -> &'static str>(b"get_message\0")
                .unwrap();
            f()
        }
    }
}

fn main() {
    let mut app: Application;
    unsafe {
        app = Application(Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error)));
    }

    let mut last_modified = std::fs::metadata(LIB_PATH).unwrap().modified().unwrap();

    let dur = std::time::Duration::from_secs(3);
    loop {
        std::thread::sleep(dur);
        println!("message: {}", app.get_message());
        drop(app);

        println!("DROPPED");
        std::thread::sleep(dur);
        unsafe {
            app = Application(
                Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error)),
            );
        }
        println!("RELOADED");
    }
}
