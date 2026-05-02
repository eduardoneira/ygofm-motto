use std::error::Error;
use std::io::{self, Write};

use ygofm_motto::CardDatabase;

fn main() -> Result<(), Box<dyn Error>> {
    let database = CardDatabase::from_bundled_csv()?;

    println!(
        "Loaded {} Yu-Gi-Oh! Forbidden Memories cards.",
        database.len()
    );
    println!("Enter a card number from 1 to 722, or q to quit.");

    loop {
        print!("card> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        let card_number = match input.parse::<u16>() {
            Ok(card_number) => card_number,
            Err(_) => {
                println!("Please enter a number from 1 to 722, or q to quit.");
                continue;
            }
        };

        match database.card(card_number) {
            Some(card) => println!("{}", database.format_card_details(card)),
            None => println!("No card found for #{card_number:03}. Try a number from 1 to 722."),
        }
    }

    Ok(())
}
