use std::env;

pub struct Config {
    pub discord_token: String,
}

impl Config {
    pub fn new() -> Self {
        let discord_token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        Config { discord_token }
    }
}