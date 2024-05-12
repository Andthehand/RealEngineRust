pub mod window;

pub struct Application {
    pub message: String,
}

impl Application {
    pub fn real_print(&self) {
        print!("Test")
    }
}
