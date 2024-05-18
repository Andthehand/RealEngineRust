//This is needed for a custom entry_point in the library
#![no_main]
extern crate real_engine;
use real_engine::core::application::Application;

#[no_mangle]
fn create_application() -> Application {
    let window_porps: real_engine::core::window::WindowProps = real_engine::core::window::WindowProps { title: String::from("Sandbox"), width: 1600, height: 900, maximized: false };
    return Application::new(window_porps);
}