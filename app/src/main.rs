//This is needed for a custom entry_point in the library
#![no_main]
extern crate real_engine;
use real_engine::{core::{application::Application, window::{self, Window}}, re_critical, re_error, re_info, re_trace, re_warn};

struct App {

}

impl Application for App {
    fn run(&self) {
        let x: i32 = 1024;

        re_trace!("{0} * 2 = {1}", x, x*2);
        re_info!("{0} * 2 = {1}", x, x*2);
        re_warn!("{0} * 2 = {1}", x, x*2);
        re_error!("{0} * 2 = {1}", x, x*2);
        re_critical!("{0} * 2 = {1}", x, x*2);

        let my_window: Box<dyn Window> = window::create();
        my_window.create_window();
    }
}

#[no_mangle]
fn create_application() -> Box<dyn Application> {
    return Box::new(App {});
}