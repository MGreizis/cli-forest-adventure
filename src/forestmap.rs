use std::io;
use crate::forestadventure::ForestAdventure;

pub struct ForestMap {
    locations: Vec<String>,
}

impl ForestMap {
    pub fn new() -> ForestMap {
        ForestMap {
            locations: vec!["lake".to_string()],
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
        println!("The path begins to become darker and darker. Are you sure you want to continue? (yes, no)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "yes" {
            println!("\nYou keep walking the path and come across a beautiful lake.\n");
            ForestAdventure::new("Forest".to_string()).start();
        } else if input == "no" {
            println!("\nYou decide to turn around and let the forest keep its secrets.\n");
            ForestAdventure::new("Forest".to_string()).start();
        } else {
            println!("Invalid command. Please try again.");
        }
    }
}
