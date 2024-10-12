// modules
pub mod app;
pub mod database;
pub mod models;

// constants
pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 9090;
pub const DATABASE_URL: &'static str = "database/raesan_base.db";
pub const DATABASE_URL_ENV_VAR: &'static str = "DATABASE_URL";
pub const PAGE_SIZE: i32 = 20;
