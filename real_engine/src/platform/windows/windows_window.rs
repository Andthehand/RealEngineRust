extern crate glfw;

use self::glfw::{Action, Context, Key};

use crate::core::window::Window;

pub struct WindowsWindow {

}

impl Window for WindowsWindow {
    fn create_window(&self) {
        let mut glfw: glfw::Glfw = glfw::init(glfw::fail_on_errors).unwrap();
    
        let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
    
        window.set_key_polling(true);
        window.make_current();
    
        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}