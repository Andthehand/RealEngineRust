use crate::core::application::Application;

extern "Rust" { 
    fn create_application() -> Box<dyn Application>; 
}

#[no_mangle]
pub fn main() {
    let app: Box<dyn Application> = unsafe { create_application() };

    app.run();
}
