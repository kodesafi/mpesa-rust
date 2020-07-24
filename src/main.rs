use dotenv;
use std::collections::HashMap;
use std::env;
use reqwest::blocking::Client;

use mpesa::{Mpesa, Environment};


fn main() {
    test().unwrap();
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let environment: Environment = "sandbox".parse()?;

    let client = Mpesa::new(
        env::var("CLIENT_KEY")?,
        env::var("CLIENT_SECRET")?,
        Environment::Sandbox, // or environment variable
        env::var("IMITATOR_PASSWORD")?,
    );

    let token = client.gen_security_credentials().unwrap();

    println!("token ==> {:?}", token);
    Ok(())
}


