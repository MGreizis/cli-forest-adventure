use std::io;
use std::sync::Once;
use crate::forestmap::ForestMap;

pub struct ForestAdventure {
    name: String,
}

static START: Once = Once::new();

impl ForestAdventure {
    pub fn new(name: String) -> ForestAdventure {
        ForestAdventure { name }
    }
    pub fn start(&self) {
        START.call_once(|| {
            println!("Welcome to the {} Adventure!\n", self.name);
        });
        ForestMap::new().display();
        println!("You are standing at the entrance of the {}.", self.name);
        println!("What would you like to do? (explore, rest, quit)");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();
            match input.as_ref() {
                "explore" => ForestMap::explore(&ForestMap::new()),
                "rest" => println!("You rest for a while."),
                "quit" => {
                    println!("Thanks for playing!");
                    break;
                }
                _ => println!("Invalid command. Please try again."),
            }
        }
    }
}
