use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write, Read, stdin};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Local;
use std::os::windows::ffi::OsStrExt;
use winapi::um::wincon::SetConsoleTitleW;
use winapi::shared::ntdef::LPCWSTR;
use term_size::dimensions;
use std::ffi::OsString;

fn generate_random_username(length: usize, mode: &str) -> String {
    let chars: Vec<char> = match mode {
        "Only Letters" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        "Letters and Numbers" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._".chars().collect(),
        "Only Numbers" => "0123456789".chars().collect(),
        "One Double Letter" | "One Triple Letter" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._".chars().collect(),
        _ => panic!("Invalid mode selected"),
    };

    let mut rng = rand::thread_rng();
    let mut username: String;

    loop {
        username = (0..length)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();

        // Check if the username meets the constraints
        let is_valid = !username.starts_with('.') && !username.ends_with('.')
            && !username.chars().all(|c| c == '.' || c == '_');

        if is_valid {
            break;
        }
    }

    if mode == "One Double Letter" {
        let duplicate_char = chars[rng.gen_range(0..chars.len())];
        let position = rng.gen_range(0..(length - 1));
        username.replace_range(position..=position + 1, &format!("{0}{0}", duplicate_char));
    } else if mode == "One Triple Letter" {
        let duplicate_char = chars[rng.gen_range(0..chars.len())];
        let position = rng.gen_range(0..(length - 2));
        username.replace_range(position..=position + 2, &format!("{0}{0}{0}", duplicate_char));
    }

    username
}

fn generate_random_usernames(count: usize, length: usize, mode: &str, results: Arc<Mutex<Vec<String>>>) {
    let mut rng = rand::thread_rng();
    let mut generated_count = 0;

    while generated_count < count {
        let username = generate_random_username(length, mode);
        let mut results = results.lock().unwrap();
        results.push(username);
        generated_count += 1;

        if generated_count % 1000 == 0 {
            print!("\rGenerated Usernames: {} / Target: {}", generated_count, count);
            io::stdout().flush().unwrap();
        }
    }

    // Final output of generated usernames
    let results = results.lock().unwrap();
    println!("\nGeneration complete. Total Usernames: {}", results.len());
}

fn save_usernames_to_file(usernames: &[String]) -> String {
    let now = Local::now();
    let filename = format!("list-{}.txt", now.format("%Y-%m-%d_%H-%M-%S"));
    let mut file = File::create(&filename).expect("Unable to create file");

    for username in usernames {
        writeln!(file, "{}", username).expect("Unable to write data");
    }

    filename
}

fn remove_duplicates_from_file(file_path: &str) -> String {
    let input_path = Path::new(file_path);

    if !input_path.exists() {
        println!("Error: The file '{}' does not exist.", file_path);
        std::process::exit(1);
    }

    let output_path = format!("cleaned_{}", input_path.display());
    let mut file_content = vec![];
    
    File::open(input_path).expect("Unable to open file")
        .read_to_end(&mut file_content).expect("Unable to read file");

    let usernames: HashSet<String> = file_content
        .split(|&byte| byte == b'\n')
        .filter_map(|line| {
            let line_str = String::from_utf8_lossy(line);
            if !line_str.is_empty() {
                Some(line_str.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut output_file = File::create(&output_path).expect("Unable to create file");
    for username in &usernames {
        writeln!(output_file, "{}", username).expect("Unable to write data");
    }

    output_path
}

fn set_console_title(title: &str) {
    let wide_title: Vec<u16> = OsString::from(title).encode_wide().chain(std::iter::once(0)).collect();
    unsafe {
        SetConsoleTitleW(wide_title.as_ptr() as LPCWSTR);
    }
}

fn print_centered_art() {
    let art = "
█▀█ 　 ▄█─ 　 ▀▀▀█ 
─▄▀ 　 ─█─ 　 ──█─ 
█▄▄ 　 ▄█▄ 　 ─▐▌─ - faster ListMaker u will every see 2 1 7";

    // Number of lines to add from the top
    let top_padding_lines = 2;

    if let Some((width, _height)) = dimensions() {
        let art_lines: Vec<&str> = art.trim().split('\n').collect();
        let max_line_length = art_lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let padding = (width - max_line_length) / 2;

        // Print empty lines for top padding
        for _ in 0..top_padding_lines {
            println!();
        }

        for line in art_lines {
            println!("{:padding$}{}", "", line, padding = padding);
        }
    } else {
        // Fallback for when terminal dimensions are unavailable
        println!("{}", art);
    }
}
fn main() {
    set_console_title("2 1 7 - LIST MAKER - @_0X0 - @undefindhash");
    print_centered_art();

    let mut input = String::new();

    println!(" [ 0 ] - Select the username type:");
    println!(" 1. Only Letters");
    println!(" 2. Letters and Numbers and ( _ & . )");
    println!(" 3. Only Numbers");
    println!(" 4. One Double Letter (e.g. tt3g)");
    println!(" 5. One Triple Letter (e.g. iiic)");
    println!(" 6. Remove Duplicates from a file");

    print!(" [ 0 ] - Enter the number of your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    match choice {
        "1" | "2" | "3" | "4" | "5" => {
            let mode = match choice {
                "1" => "Only Letters",
                "2" => "Letters and Numbers",
                "3" => "Only Numbers",
                "4" => "One Double Letter",
                "5" => "One Triple Letter",
                _ => panic!("Invalid choice"),
            };

            input.clear();
            print!(" [ 0 ] - How many usernames do you want to generate? : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let total_count: usize = input.trim().parse().expect("Please enter a valid number");

            input.clear();
            print!(" [ 0 ] - What should be the length of each username? : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let length: usize = input.trim().parse().expect("Please enter a valid number");

            let results = Arc::new(Mutex::new(Vec::new()));
            let results_clone = Arc::clone(&results);

            let handle = thread::spawn(move || {
                generate_random_usernames(total_count, length, mode, results_clone);
            });

            handle.join().expect("Thread panicked");

            let results = results.lock().unwrap();
            let filename = save_usernames_to_file(&results);
            println!("\nCompleted task. Usernames saved to file: {}", filename);
        }
        "6" => {
            loop {
                input.clear();
                print!("[ 0 ] - Enter the filename to remove duplicates from: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let file_name = input.trim();

                let input_path = Path::new(file_name);

                if input_path.exists() {
                    let cleaned_file = remove_duplicates_from_file(file_name);
                    println!("\nCompleted task. Duplicates removed. Cleaned list saved to file: {}", cleaned_file);
                    break;
                } else {
                    println!("Error: The file '{}' does not exist. Please try again.", file_name);
                }
            }
        }
        _ => panic!("Invalid choice, please select 1, 2, 3, 4, 5, 6"),
    }

    println!("\nPress Enter to exit...");
    stdin().read_line(&mut String::new()).unwrap();
}

