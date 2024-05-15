//WHY!!!!!!!!
//I don't want this exposed outside the library
mod platform {
    pub mod windows {
        pub mod windows_window;
    }
}

pub mod core {
    pub mod window;
}

pub struct Application {
    pub message: String,
}

impl Application {
    pub fn real_print(&self) {
        print!("Test")
    }
}
