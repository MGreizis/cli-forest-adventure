use crate::forestmap::ForestMap;
use crate::merchant::Merchant;
use crate::questgiver::QuestGiver;
use std::io;
use std::sync::Once;

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
            ForestMap::new().display();
        });
        println!("You are standing at the entrance of the {}.", self.name);
        println!("What would you like to do? (explore, interact, rest, quit)");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();
            println!("");
            match input.as_ref() {
                "explore" => ForestMap::explore(&ForestMap::new()),
                "rest" => println!("You rest for a while."),
                "interact" => {
                    println!("Which NPC would you like to interact with? (questgiver, merchant)\n");
                    let mut npc_input = String::new();
                    io::stdin().read_line(&mut npc_input).unwrap();
                    let npc_input = npc_input.trim().to_string();
                    if npc_input == "questgiver" {
                        QuestGiver::interact();
                    } else if npc_input == "merchant" {
                        Merchant::interact();
                    } else {
                        println!("Invalid command. Please try again.");
                    }
                }
                "quit" => {
                    println!("Thanks for playing!");
                    break;
                }
                _ => println!("Invalid command. Please try again."),
            }
        }
    }
}
