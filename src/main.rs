mod forestadventure;

fn main() {
    let adventure = forestadventure::ForestAdventure::new("Forest".to_string());
    adventure.start();
}
