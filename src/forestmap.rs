use std::io;
// use crate::forestadventure::ForestAdventure;

pub struct ForestMap {
    locations: Vec<String>,
}

impl ForestMap {
    pub fn new() -> ForestMap {
        ForestMap {
            locations: vec!["lake".to_string(), "cave".to_string()],
        }
    }

    pub fn display(&self) {
        println!("Locations in the forest:");
        for location in &self.locations {
            println!("- {}", location);
        }
    }

    pub fn explore(&self) {
        println!("You start to explore the forest.");
        println!("You come across a fork in the road. Which path do you want to take? (left, right)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "left" {
            println!("\nYou take the left path and come across a beautiful lake.\n");
        } else if input == "right" {
            println!("\nYou decide to take the right path and end up at a dark and mysterious cave.\n");
        } else {
            println!("Invalid command. Please try again.");
        }
    }
}
