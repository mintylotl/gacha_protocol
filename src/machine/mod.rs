mod table;

use crate::Player;
pub use table::Rarities;

pub fn handle_pull(player: &mut Player) -> Rarities {
    Rarities::pull(player)
}
pub fn handle_pull_ten(player: &mut Player) -> Vec<Rarities> {
    let mut wishes = Vec::new();

    for _ in 0..10 {
        wishes.push(Rarities::pull(player));
    }
    wishes
}
