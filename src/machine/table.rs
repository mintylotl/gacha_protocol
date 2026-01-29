use crate::Player;
use rand::{self, Rng};

pub enum Rarities {
    MythicSSS,
    SS,
    S,
    A,
    B,
    C,
    NoTickets,
}

impl Rarities {
    pub fn pull(player: &mut Player) -> Rarities {
        if player.tickets < 1 {
            return Rarities::NoTickets;
        } else {
            player.tickets -= 1 as u32;
        }
        let mut rng = rand::rng();
        let roll: f64 = rng.random();

        // 0.072% of hitting Mythic
        if roll < 0.00072 + player.accumulated_increase as f64 {
            return Rarities::MythicSSS;
        }
        // 0.72% chance of hitting SS
        if roll < 0.00792 {
            return Rarities::SS;
        }
        // 7.2% chance of hitting S
        if roll < 0.07992 {
            return Rarities::S;
        }
        // 16% chance of hitting A
        if roll < 0.23992 {
            return Rarities::A;
        }
        // 24% chance of hitting B
        if roll < 0.47992 {
            return Rarities::B;
        }
        // 52% chance of hitting C
        else {
            return Rarities::C;
        }
    }
}
