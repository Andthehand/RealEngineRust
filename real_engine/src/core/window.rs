use crate::platform::windows::windows_window::WindowsWindow;

pub trait Window {
    fn create_window(&self);
}

pub fn create() -> Box<dyn Window> {
    return Box::new(WindowsWindow {})
}