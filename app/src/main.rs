//This is needed for a custom entry_point in the library
#![no_main]
extern crate real_engine;
use real_engine::core::application::Application;

struct App {

}

impl Application for App {
    fn get_window_props(&self) -> real_engine::core::window::WindowProps {
        return real_engine::core::window::WindowProps { title: String::from("Sandbox"), width: 1600, height: 900, maximized: false }
    }
}

#[no_mangle]
fn create_application() -> Box<dyn Application> {
    return Box::new(App {});
}