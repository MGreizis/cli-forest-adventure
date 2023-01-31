mod forestadventure;
mod forestmap;
mod questgiver;
mod merchant;

fn main() {
    let adventure = forestadventure::ForestAdventure::new("Forest".to_string());
    adventure.start();
}
