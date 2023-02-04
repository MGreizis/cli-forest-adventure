use std::io;
use crate::forestadventure::ForestAdventure;

pub struct ForestMap {
    locations: Vec<String>,
    is_artifact_found: bool,
}

impl ForestMap {
    pub fn new() -> ForestMap {
        ForestMap {
            locations: vec!["lake".to_string(), "cave".to_string()],
            is_artifact_found: false,
        }
    }

    pub fn display(&self) {
        println!("Locations in the forest:");
        for location in &self.locations {
            println!("- {}", location);
        }
    }

    pub fn explore(&mut self) {
        println!("You start to explore the forest.");
        println!("You come across a fork in the road. Which path do you want to take? (left, right)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "left" {
            println!("You take the left path and come across a beautiful lake.\n");
            ForestAdventure::start(&ForestAdventure::new("Forest".to_string()));
        } else if input == "right" {
            println!("You decide to take the right path and end up at a dark and mysterious cave.\n");
            if !self.is_artifact_found {
                println!("This must be the cave where the lost artifact is said to be.\n");
                println!("Do you wish to enter the cave? (yes, no)\n");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                if input == "yes" {
                    println!("You enter the cave and find the lost artifact!\n");
                    self.is_artifact_found = true;
                } else {
                    println!("You decide not to enter the cave.\n");
                }
            } else {
                println!("You have already found the lost artifact.\n");
            }
            ForestAdventure::start(&ForestAdventure::new("Forest".to_string()));
        } else {
            println!("Invalid command. Please try again.");
        }
    }
}
