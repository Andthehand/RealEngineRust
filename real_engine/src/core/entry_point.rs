use crate::core::application::Application;
use crate::core::log::Log;

extern "Rust" { 
    fn create_application() -> Box<dyn Application>; 
}

#[no_mangle]
pub fn main() {
    Log::print();

    let app: Box<dyn Application> = unsafe { create_application() };

    app.run();
}
