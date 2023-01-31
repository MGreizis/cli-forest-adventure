use crate::forestadventure::ForestAdventure;

pub struct Merchant {
    items_for_sale: Vec<String>,
    name: String,
    dialogue: String,
}

impl Merchant {
    pub fn new(name: String, dialogue: String, items_for_sale: Vec<String>) -> Self {
        Self { name, dialogue, items_for_sale }
    }

    pub fn show_items(&self) {
        println!( "{} {} {}\n", self.name, self.dialogue, self.items_for_sale.join(", \n"));
    }

    pub fn interact() {
        Merchant::new(
            "\nMerchant:\n".to_string(),
            "I've got various wares, if you've got coin.\n".to_string(),
            vec![
                "Potion".to_string(),
                "Sword".to_string(),
                "Shield".to_string(),
            ],
        ).show_items();
        ForestAdventure::start(&ForestAdventure::new("Forest".to_string()));
    }
}
