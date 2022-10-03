use std::fs::{self, File};
use std::io::{self, Write};
use std::process::{self, Command};
use anyhow::{Context, Result};
use rand::Rng;

pub fn start() -> Result<u32> {
    println!("Welcome to Anki written in rust");
    let command = loop {
        println!("1. Create new package");
        println!("2. Open package");
        println!("3. List all packages");
        let mut command: String = String::new();
        let x = io::stdin().read_line(&mut command);
        if let Err(e) = x {
            println!("Failed to read line with this error: {}", e);
            println!("Try again: ");
            continue;
        };

        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("You didn't input a number");
                println!("Failed with this error: {}", e);
                println!("Try again: ");
                continue;
            }
        };

        if command > 3 {
            println!("You have to input either 1, 2 or 3");
            println!("Try again: ");
            continue;
        }

        break command;
    };
    Ok(command)
}

pub fn y_or_n() -> String {
    let command: String = loop {
        let mut com = String::new();
        if let Err(e) = io::stdin().read_line(&mut com) {
            println!("Failed with this error: {}", e);
            println!("Try again");
            println!("Y/n");
            continue;
        }

        if com.trim().len() > 1 {
            println!("Input either Y or n");
            println!("Try again");
            println!("Y/n");
            continue;
        }
        break com;
    };
    command
}

pub fn dispatch(command: u32) -> Result<()> {
    match command {
        1 => {
            create_package()?;
            process::exit(1);
        }
        2 => {
            Command::new("clear").status().unwrap();
            list_all_packages()?;
            println!("Which one do you want to open?");
            let mut package = String::new();
            io::stdin().read_line(&mut package)?;
            Command::new("clear").status().unwrap();
            let exists: bool = check_if_package_exists(&package)?;

            if exists == false {
                println!("This package doesn't exist.");
                println!("Do you want to create it?");
                println!("Y/n");
                loop {
                    let mut command = String::new();
                    if let Err(e) = io::stdin().read_line(&mut command) {
                        println!("Failed with this error: {}", e);
                        println!("Try again");
                        println!("Y/n");
                        continue;
                    }

                    if command.trim().len() > 1 {
                        println!("Input either Y or n");
                        println!("Try again");
                        println!("Y/n");
                    }

                    if command.to_lowercase().contains("y") {
                        create_package()?;
                        process::exit(1);
                    } else {
                        process::exit(1);
                    }
                }
            }

            let command = loop {
                println!("Press 1 To add more flashcards");
                println!("Press 2 To practice");
                let mut command = String::new();
                if let Err(e) = io::stdin()
                    .read_line(&mut command)
                    .context("Failed to read line")
                {
                    println!("Couldn't read line because of this error: {}", e);
                    println!("Try again");
                    continue;
                }

                let command: u32 = match command.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Failed with this error: {}", e);
                        println!("Try again");
                        continue;
                    },
                };

                if command > 2 {
                    println!("Input has to be either 1 or 2");
                    println!("Try again");
                    continue;
                }
                break command;
            };

            if command == 1 {
                loop {
                    Command::new("clear").status().unwrap();
                    add_flashcards(&package)?;
                    println!("Do you want to add more flashcards?");
                    println!("Y/n");
                    let command = y_or_n();
                    if command.to_lowercase().trim().contains("y") {
                        continue;
                    } else {
                        process::exit(1);
                    }
                }
            } else {
                Command::new("clear").status().unwrap();
                let path = format!("data/{}.txt", package.trim());

                let file = fs::read_to_string(path)?;
                show_random_flashcard(&file)?;
            }
            
        }
        3 => {
            Command::new("clear").status().unwrap();
            list_all_packages()?;
        }
        _ => {}
    };

    Ok(())
}

pub fn list_all_packages() -> Result<()> {
    println!("These are all your packages: \n");
    let file = fs::read_to_string("data/database.txt").context("Failed to read file")?;
    println!("{}", file);
    Ok(())
}

pub fn create_package() -> Result<()> {
    let name = loop {
        let mut name = String::new();
        println!("Input name of your new package: ");
        io::stdin()
            .read_line(&mut name)
            .context("Failed to read line")?;
        let exists = check_if_package_exists(&name.trim())?;
        if exists == true {
            println!("This package already exists");
            println!("Try again");
            continue;
        }
        break name;
    };

    println!("Creating new package...");
    File::create(format!("data/{}.txt", &name.trim())).context("Failed to create file")?;
    println!("Package {} created", name.trim());

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("data/database.txt")
        .context("Failed to open database.txt")?;
    writeln!(file, "{}", name).context("Failed to write to database.txt")?;

    Ok(())
}

pub fn check_if_package_exists(name: &str) -> Result<bool> {
    let file = fs::read_to_string("data/database.txt")?;
    if file.contains(name) {
        return Ok(true);
    }
    Ok(false)
}


pub fn add_flashcards(package: &str) -> Result<()> {
    println!("Writing to {}", package);

    let front_side = loop {
        println!("Front side: ");
        let mut front = String::new();
        if let Err(e) = io::stdin().read_line(&mut front) {
            println!("Failed with this error: {}", e);
            println!("Try again");
            continue;
        }

        break front;
    };
    let back_side = loop {
        println!("Back side: ");
        let mut back = String::new();
        if let Err(e) = io::stdin().read_line(&mut back) {
            println!("Failed with this error: {}", e);
            println!("Try again");
            continue;
        }
        break back;
    };

    let path = format!("data/{}.txt", package.trim());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .with_context(|| format!("Failed to write to {}", package))?;

    let to_write = format!("{} | {}", front_side.trim(), back_side.trim());

    writeln!(file, "{}", to_write)?;

    Ok(())
}


pub fn show_random_flashcard(file: &String) -> Result<()> {
    let mut rounds = 0;
    loop {

        if file.is_empty() {
            println!("You need to add some flashcards");
            process::exit(1);
        }
        let count = file.lines().count();

        let rand_num = rand::thread_rng().gen_range(0..=count);

        let mut word = String::new();

        let mut lines = file.lines();

        let mut index = 0;
        while let Some(line) = lines.next() {
            if index == rand_num {
                if line.is_empty() {
                    word = lines.next().unwrap().to_string();
                    if word.is_empty() {
                        println!("End of the list");
                        println!("Exiting...");
                        process::exit(1);
                    }
                    break;
                } else {
                    word = line.to_string();
                    break;
                }
            } 
            index += 1;
        }

        let rand_num = rand::thread_rng().gen_range(0..1);

        let words = word.split("|");
        let both_sides = words.collect::<Vec<_>>();
        println!("{}", both_sides[rand_num]);

        println!("Show answer?");
        println!("Y/n");

        let command = y_or_n();

        if command.contains("n") {
            println!("Exiting...");
            process::exit(1);
        }

        if rand_num == 1 {
            println!("{}", both_sides[0].trim());
        } else {
            println!("{}", both_sides[1].trim());
        }

        println!("###############################");

        if rounds == count {
            break;
        }

        rounds += 1;
    }
    Ok(())
}

