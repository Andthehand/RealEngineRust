use crate::core::window::{self, Window};

use super::window::WindowProps;

pub struct Application {
    window: Box<dyn Window>
}

impl Application {
    pub fn new(window_props: WindowProps) -> Self{
        let my_window: Box<dyn Window> = window::create(window_props);

        return Application { window: my_window };
    }
}
