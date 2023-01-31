mod forestadventure;
mod forestmap;
mod questgiver;

fn main() {
    let adventure = forestadventure::ForestAdventure::new("Forest".to_string());
    adventure.start();
}
