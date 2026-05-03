use std::error::Error;
use std::io::{self, Write};

mod gui;

use ygofm_motto::CardDatabase;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|argument| argument == "--cli") {
        return run_cli();
    }

    gui::run_card_tracker()
}

fn run_cli() -> Result<(), Box<dyn Error>> {
    let database = CardDatabase::from_bundled_csv()?;

    println!(
        "Loaded {} Yu-Gi-Oh! Forbidden Memories cards and {} duelists.",
        database.len(),
        database.duelists().len()
    );
    print_help();

    loop {
        print!("command> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input == "q" {
            break;
        }

        if input == "help" {
            print_help();
            continue;
        }

        let parts = input.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["duelists"] => {
                for duelist in database.duelists() {
                    println!("#{:02} {}", duelist.id, duelist.name);
                }
            }
            ["card", card_input] => {
                let card_number = match card_input.parse::<u16>() {
                    Ok(card_number) => card_number,
                    Err(_) => {
                        println!("Please enter a card number from 1 to 722.");
                        continue;
                    }
                };

                match database.card(card_number) {
                    Some(card) => println!("{}", database.format_card_details(card)),
                    None => {
                        println!("No card found for #{card_number:03}. Try a number from 1 to 722.")
                    }
                }
            }
            ["duelist", duelist_input] => {
                let duelist_number = match duelist_input.parse::<u8>() {
                    Ok(duelist_number) => duelist_number,
                    Err(_) => {
                        println!("Please enter a duelist number from 1 to 39.");
                        continue;
                    }
                };

                match database.duelist(duelist_number) {
                    Some(duelist) => println!("{}", database.format_duelist_details(duelist)),
                    None => println!(
                        "No duelist found for #{duelist_number:02}. Try a number from 1 to 39."
                    ),
                }
            }
            _ => {
                println!("Unknown command.");
                print_help();
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Commands:");
    println!("  card <number>     Show card details, for example card 35");
    println!("  duelist <number>  Show opponent deck and drop pools, for example duelist 1");
    println!("  duelists          List all duelists");
    println!("  q                 Quit");
}
