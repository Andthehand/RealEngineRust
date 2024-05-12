

pub struct Application {
    pub message: String,
}

impl Application {
    pub fn real_engine_print(&self) {
        println!("{}", &self.message);
    }
}
