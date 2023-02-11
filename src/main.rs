use std::{io::Write, clone};

fn main() {
    let console_input = std::io::stdin();
    let mut console_output = std::io::stdout();

    let mut line = String::new();

    loop {
        print!("ʇ> ");
        console_output.flush().unwrap();

        console_input.read_line(&mut line).unwrap();

        let split_input = line.split_whitespace().collect::<Vec<_>>();
        let mut cleaned_split: Vec<String> = Vec::new();

        let mut in_quotes = false;
        let mut token_acc = String::new();
        for token in split_input.iter() {
            let mut token_str = token.to_string();
            if !in_quotes {
                if token_str.starts_with("\"") {
                    in_quotes = true;
                    token_str = token_str[1..].to_string();
                }
            }

            if in_quotes {
                if token_str.ends_with("\"") {
                    let mut escaped = false;
                    for char in token_str.chars() {
                        if char == '\\' {
                            escaped = !escaped;
                        }
                    }

                    if !escaped {
                        in_quotes = false;
                        token_str = token_str[..token_str.len()-1].to_string();
                    }
                }


            }

            token_acc.push_str(&token_str);

            if !in_quotes {
                let cloned = token_acc.clone();
                cleaned_split.push(cloned);
                token_acc.clear();
            }
        }

        let command = cleaned_split.first().unwrap();
        if command == "exit" {
            return;
        } else if command == "ls" {
            if cleaned_split.len() > 1 {
                ls(cleaned_split.get(1).unwrap());
            } else {
                ls("./");
            }
        } else {
            println!("Unrecognized command");
        }

        line.clear();
    }
}

fn ls(folder_path: &str) {
    let paths = std::fs::read_dir(folder_path).unwrap();

    println!("Files in {}", folder_path);

    for path in paths {
        println!("\t{}", path.unwrap().file_name().to_str().unwrap());
    }
}