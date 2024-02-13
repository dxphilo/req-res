use dotenvy::dotenv;
use std::env;

pub fn load_config() {
    println!("Loading env variables...");
    dotenv().ok();
    println!("Env ariables loaded")
}

pub fn app_port() -> u16 {
    let port = env::var("PORT").expect("PORT is not set in.env file");
    port.parse::<u16>().expect("PORT is not a number")
}
