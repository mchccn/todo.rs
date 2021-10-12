use std::env;
use std::fs;
use std::path::MAIN_SEPARATOR;

fn main() {
    let home_path = match env::home_dir() {
        Some(path) => path,
        None => {
            println!("No home directory found. Exiting...");
            std::process::exit(1);
        }
    };

    let home_dir = home_path.to_str().unwrap();

    let config_dir = format!("{}{}{}", home_dir, MAIN_SEPARATOR, ".config");

    let data_dir = format!("{}{}{}", config_dir, MAIN_SEPARATOR, "rust-todo");

    let data_file = format!("{}{}{}", data_dir, MAIN_SEPARATOR, "data.txt");

    if match fs::metadata(&config_dir) {
        Ok(_) => false,
        Err(_) => true,
    } {
        match fs::create_dir(&config_dir) {
            Ok(_) => (),
            Err(_) => {
                println!("Unable to create config directory.");
                std::process::exit(1);
            }
        }
    }

    if match fs::metadata(&data_dir) {
        Ok(_) => false,
        Err(_) => true,
    } {
        match fs::create_dir(&data_dir) {
            Ok(_) => (),
            Err(_) => {
                println!("Unable to create data directory.");
                std::process::exit(1);
            }
        }
    }

    if match fs::metadata(&data_file) {
        Ok(_) => false,
        Err(_) => true,
    } {
        match fs::write(&data_file, "") {
            Ok(_) => (),
            Err(_) => {
                println!("Unable to create data file");
                std::process::exit(1);
            }
        }
    }

    let argv = env::args();

    let bytes = match fs::read(&data_file) {
        Ok(data) => data,
        Err(_) => {
            println!("Unable to read data file");
            std::process::exit(1);
        }
    };

    let data: String = String::from_utf8_lossy(&bytes).parse().unwrap();

    if argv.len() == 1 {
        println!("{}", data);

        std::process::exit(0);
    }

    let mut todos = data.split("\n").collect::<Vec<&str>>();

    if argv.skip(1).collect::<Vec<String>>().first().unwrap() == "new" {
        let todo = env::args().skip(1).collect::<Vec<String>>().join(" ");

        todos.push(&todo);

        fs::write(&data_file, todos.join("\n")).expect("Failed to save todos.");

        println!("New todo created!");

        std::process::exit(0);
    }
}
