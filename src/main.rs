mod forestadventure;
mod forestmap;

fn main() {
    let adventure = forestadventure::ForestAdventure::new("Forest".to_string());
    adventure.start();
}
