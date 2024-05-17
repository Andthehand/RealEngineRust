//I don't want this exposed outside the library
mod platform {
    pub mod opengl {
        pub mod opengl_context;
    }

    pub mod windows {
        pub mod windows_window;
    }
}

pub mod core {
    pub mod application;
    pub mod window;
    pub mod entry_point;
    pub mod log;
    pub mod assert;
}