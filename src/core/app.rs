// ----- `Config` object
pub struct Config {
    pub port: u16,
    pub address: String,
}

// ----- `Application` object
pub struct Application {
    pub config: Config,
}
impl Application {
    pub fn new(config: Config) -> Application {
        return Application { config };
    }
}
