//This is needed for a custom entry_point in the library
#![no_main]
extern crate real_engine;
use real_engine::{core::window::{self, Window}, core::application::Application};

struct App {

}

impl Application for App {
    fn run(&self) {
        let my_window: Box<dyn Window> = window::create();
        my_window.create_window();
    }
}

#[no_mangle]
fn create_application() -> Box<dyn Application> {
    return Box::new(App {});
}