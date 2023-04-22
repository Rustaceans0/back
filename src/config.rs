use dotenv::dotenv;
use std::env;

pub struct server_config{
    pub api_url: String,
    pub host: u32,
    pub port: u32,
}
pub fn get_config() -> server_config {
    dotenv().ok();
    let api_url = env::var("sk-...kxmv").unwrap();
    let host = env::var("HOST").unwrap().parse::<u32>().unwrap();
    let port = env::var("PORT").unwrap().parse::<u32>().unwrap();
    server_config {
        api_url,
        host,
        port,
    }
}