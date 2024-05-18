use crate::core::application::Application;
use crate::re_core_trace;


extern "Rust" { 
    fn create_application() -> Box<dyn Application>; 
}

#[no_mangle]
pub fn main() {
    re_core_trace!("Starting RealEngine");

    let app: Box<dyn Application> = unsafe { create_application() };

    //Creates a window and inits opengl
    app.init();
}
