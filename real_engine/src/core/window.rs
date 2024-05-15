use crate::platform::windows::windows_window::WindowsWindow;

pub trait Window {
    fn create_window(&self);
}

//This will error if compiling for anything other than windows
pub fn create() -> Box<dyn Window> {
    #[cfg(target_os = "windows")]
    return Box::new(WindowsWindow {});
}