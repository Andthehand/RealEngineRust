extern crate real_engine;
use real_engine::Application;
use real_engine::window;

fn main() {
    let app = Application{ message: String::from("Hello RealEngine!") };

    app.real_print();

    window::create_window();
}