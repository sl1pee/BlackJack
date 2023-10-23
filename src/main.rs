use rand::prelude::*;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let pulled_number = rng.gen_range(1..=21);
    let mut dealer = rng.gen_range(1..=30);
    let mut user_total = pulled_number;

    println!("You pulled a: {}", pulled_number);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error: Failed to read input.");
        let input = input.trim().to_lowercase();


        if input == "stand" {
            println!("Standing at: {}", user_total);
            break;
        } else if input == "hit" {
            let additional_card = rng.gen_range(1..=21);
            user_total += additional_card;
            println!("You pulled: {}", additional_card);
            println!("Total: {}", user_total);
            if user_total > 21 {
                println!("All Gamblers quit before they win big");
                println!("You Lose");
                return;
            }
        } else {
            println!("Invalid input. Type 'stand' or 'hit'.");
        }
    }

    println!("Dealer's Turn");
    println!("The Dealer pulled: {}", dealer);

    while dealer < 17 {
        let dealer_card = rng.gen_range(1..=30);
        dealer += dealer_card;
        println!("Dealer pulled: {}", dealer_card);
    }

    if dealer > 21 {
        println!("Dealer Busted");
    } else if dealer >= user_total {
        println!("Dealer Wins");
    } else {
        println!("You Win");
    }
}
