extern crate real_engine;
use real_engine::{core::window::Window, Application};

fn main() {
    let app: Application = Application{ message: String::from("Hello RealEngine!") };

    app.real_print();

    let window: Box<dyn Window> = real_engine::core::window::create();
    window.create_window();
}