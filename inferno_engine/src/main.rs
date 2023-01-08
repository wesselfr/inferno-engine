use inferno_engine::{engine_draw, reload::*};
use shared::State;
use std::time::SystemTime;

fn main() {
    let mut test = State {
        version: 1,
        draw_fn: engine_draw,
    };

    let mut app: Application;
    app = load_lib();

    let mut last_modified = SystemTime::now();

    loop {
        println!("message: {}", app.get_message());
        app.update(&test);

        if should_reload(last_modified) {
            println!("== NEW VERSION FOUND ==");
            app = reload(app);
            println!("== NEW VERSION LOADED ==");
            test.version += 1;
            last_modified = SystemTime::now();
            app.setup(&test);
            app.update(&test);
        }
    }
}
