use std::error::Error;
use std::io::{self, Write};

use ygofm_motto::CardDatabase;

fn main() -> Result<(), Box<dyn Error>> {
    let database = CardDatabase::from_bundled_csv()?;

    println!(
        "Loaded {} Yu-Gi-Oh! Forbidden Memories cards and {} duelists.",
        database.len(),
        database.duelists().len()
    );
    println!("Enter a card number from 1 to 722, duelist <number>, duelists, or q to quit.");

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

        if input.eq_ignore_ascii_case("duelists") {
            for duelist in database.duelists() {
                println!("#{:02} {}", duelist.id, duelist.name);
            }
            continue;
        }

        if let Some(duelist_input) = input
            .strip_prefix("duelist ")
            .or_else(|| input.strip_prefix("d "))
        {
            let duelist_number = match duelist_input.trim().parse::<u8>() {
                Ok(duelist_number) => duelist_number,
                Err(_) => {
                    println!("Please enter a duelist number from 1 to 39.");
                    continue;
                }
            };

            match database.duelist(duelist_number) {
                Some(duelist) => println!("{}", database.format_duelist_deck(duelist)),
                None => println!(
                    "No duelist found for #{duelist_number:02}. Try a number from 1 to 39."
                ),
            }
            continue;
        }

        let card_number = match input.parse::<u16>() {
            Ok(card_number) => card_number,
            Err(_) => {
                println!(
                    "Please enter a card number from 1 to 722, duelist <number>, duelists, or q to quit."
                );
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
