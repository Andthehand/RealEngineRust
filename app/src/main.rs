extern crate real_engine;
use real_engine::Application;

fn main() {
    let app = Application{ message: String::from("Hello RealEngine!") };

    app.real_engine_print();
}