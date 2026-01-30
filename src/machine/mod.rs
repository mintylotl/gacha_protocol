mod table;

use crate::Player;
pub use table::get_string;
pub use table::Rarities;

pub fn handle_pull(player: &mut Player) -> Vec<Rarities> {
    let mut vec = Vec::new();
    vec.push(Rarities::pull(player));
    vec
}
pub fn handle_pull_ten(player: &mut Player) -> Vec<Rarities> {
    let mut wishes = Vec::new();

    for _ in 0..10 {
        wishes.push(Rarities::pull(player));
    }
    wishes
}
pub fn handle_pull_h(player: &mut Player) -> Vec<Rarities> {
    let mut wishes = Vec::new();

    for _ in 0..10000 {
        wishes.push(Rarities::pull(player));
    }
    wishes
}
