use crate::platform::windows::windows_window::WindowsWindow;

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

pub trait Window {

}

//This will error if compiling for anything other than windows
pub fn create(window_props: WindowProps) -> Box<dyn Window> {
    #[cfg(target_os = "windows")]
    return Box::new(WindowsWindow::new(window_props));
}