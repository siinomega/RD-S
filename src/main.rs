use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use colored::Colorize;
use reqwest::Client;
use tokio;
use indicatif::ProgressBar;

#[tokio::main]
async fn main() {
    // Rust Dir Scan
    let client = Client::new();
    let banner = reqwest::get("https://gist.githubusercontent.com/siinomega/5bbfe0b73da35f5ceda90a8f035fdc71/raw")
        .await.unwrap().text().await.unwrap();
    println!("{}\n", banner.yellow());
    let base_url = loop {
        let mut base_url = String::new();
        print!("Veuillez Entrer Une URL A Tester : ");
        stdout().flush().unwrap();
        stdin().read_line(&mut base_url).unwrap();
        let base_url = base_url.trim().to_string();
        if base_url.contains("http://") || base_url.contains("https://") {
            break base_url;
        } else {
            let invalid = "Veuillez Specifier Une URL Valide";
            println!("{}", invalid.red());
        }
    };

    let wordlist_path = loop {
        print!("Veuillez Spécifier Le Path De Votre Wordlist : ");
        stdout().flush().unwrap();
        let mut path = String::new();
        stdin().read_line(&mut path).unwrap();
        let path = path.trim().to_string();

        if path.contains(".") {
            match File::open(&path) {
                Ok(_) => break path,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        } else {
            let error = "Veuillez Spécifier Un Path Valide !";
            println!("{}", error.red());
        }
    };

    if let Ok(file) = File::open(wordlist_path) {
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
        let total_paths = lines.len();
        let pb = ProgressBar::new(total_paths as u64);
        let mut found_paths = vec![];

        for word in lines {
            let base_url = base_url.trim_end_matches('/');
            let full_url = format!("{}/{}", base_url, word);
            match client.get(&full_url).send().await {
                Ok(response) => {
                    if response.status().as_u16() == 200 || response.status().as_u16() == 302 {
                        found_paths.push(full_url.clone());
                    }
                }
                Err(err) => {
                    eprintln!("Error with {:?}: {:?}", full_url, err);
                }
            }
            pb.inc(1);
        }
        pb.finish_with_message("Scanning Complete!");
        if !found_paths.is_empty() {
            println!("\nFound Paths:");
            for path in found_paths {
                println!("{}", path);
            }
        }
    }
}
