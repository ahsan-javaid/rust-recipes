extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Raffle Program!");
    
    // Collect participants' names
    let mut participants = Vec::new();
    loop {
        println!("Enter a participant's name (or type 'done' to finish):");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim();
        
        if name == "done" {
            break;
        }
        
        participants.push(name.to_string());
    }
    
    // Check if there are enough participants to hold a raffle
    if participants.is_empty() {
        println!("No participants entered. Exiting.");
        return;
    }
    
    // Generate a random winner
    let mut rng = rand::thread_rng();
    let winner_index = rng.gen_range(0..participants.len());
    let winner = &participants[winner_index];
    
    println!("And the winner is... {}!", winner);
}
