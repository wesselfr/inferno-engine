extern crate libloading;

use std::fs;

use libloading::Library;

const LIB_PATH: &'static str = "../app/target/debug/app.dll";
const LIB_PATH_ACTIVE: &'static str = "active/app.dll";

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
    fs::copy(LIB_PATH, LIB_PATH_ACTIVE).unwrap_or_else(|error| panic!("{}", error));
    unsafe {
        app = Application(Library::new(LIB_PATH_ACTIVE).unwrap_or_else(|error| panic!("{}", error)));
    }

    let mut last_modified = std::fs::metadata(LIB_PATH_ACTIVE).unwrap().modified().unwrap();

    let dur = std::time::Duration::from_secs(3);
    loop {
        std::thread::sleep(dur);
        println!("message: {}", app.get_message());
        
        // Check if the DLL at the compile location is newer.
        let modified = std::fs::metadata(LIB_PATH).unwrap().modified().unwrap();

        if modified > last_modified
        {
            println!("== NEW VERSION FOUND ==");
            drop(app);
            fs::copy(LIB_PATH, LIB_PATH_ACTIVE).unwrap();

            unsafe {
                app = Application(
                    Library::new(LIB_PATH_ACTIVE).unwrap_or_else(|error| panic!("{}", error)),
                );
            }
            last_modified = modified;
            println!("== NEW VERSION LOADED ==");
        }
    }
}
