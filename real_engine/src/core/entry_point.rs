use crate::core::application::Application;
use crate::re_core_trace;


extern "Rust" { 
    fn create_application() -> Application; 
}

#[no_mangle]
pub fn main() {
    re_core_trace!("Starting RealEngine");

    let app: Application = unsafe { create_application() };
}
