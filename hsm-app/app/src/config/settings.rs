use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub addr: String,
	pub auth_token: String,
	pub key_name: String,
}

impl Config {
	pub fn new() -> Self {
		dotenv().ok();

		let addr = match env::var("ADDR") {
			Ok(value) => value,
			Err(_) => {
				eprintln!("Error: ADDR is not set in the .env file.");
				std::process::exit(1);
			}
		};

		let auth_token = match env::var("AUTH_TOKEN") {
			Ok(value) => value,
			Err(_) => {
				eprintln!("Error: AUTH_TOKEN is not set in the .env file.");
				std::process::exit(1);
			}
		};

		let key_name = match env::var("KEY_NAME") {
			Ok(value) => value,
			Err(_) => {
				eprintln!("Error: KEY_NAME is not set in the .env file.");
				std::process::exit(1);
			}
		};

		Config { addr, auth_token, key_name }
	}
}
