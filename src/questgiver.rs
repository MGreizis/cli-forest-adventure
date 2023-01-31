use crate::forestadventure::ForestAdventure;

pub struct QuestGiver {
    name: String,
    dialogue: String,
    quest: String,
}

impl QuestGiver {
    pub fn new(name: String, dialogue: String, quest: String) -> Self {
        Self { name, dialogue, quest }
    }

    pub fn give_quest(&self) {
        println!("{} {} {}", self.name, self.dialogue, self.quest);
    }

    pub fn interact() {
        QuestGiver::new(
            "Eldrin the Elf:\n".to_string(),
            "Greetings, adventurer! I have a quest that I believe you are well suited for."
                .to_string(),
            "Retrieve the lost artifact from the cave.\n".to_string(),
        ).give_quest();
            ForestAdventure::start(&ForestAdventure::new("Forest".to_string()));
    }
}