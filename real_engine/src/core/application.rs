use crate::core::window::{self, Window};

use super::window::WindowProps;

pub trait Application {
    fn init(&self) {
        let my_window: Box<dyn Window> = window::create();
        my_window.create_window(self.get_window_props());
    }

    fn get_window_props(&self) -> WindowProps;
}