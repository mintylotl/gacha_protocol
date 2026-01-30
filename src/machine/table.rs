use std::iter;

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
            player.tickets -= 1;
            player.total_pulls += 1;
            player.sss_pity += 1;
            player.ss_pity += 1;
            player.s_pity += 1;
        }

        let mythic = 0.00072 + player.inc_sss;
        let ss = mythic + 0.0048 + player.inc_ss;
        let s = ss + 0.024 + player.inc_s;
        let a = s + 0.072;
        let b = a + 0.24;

        let mut rng = rand::rng();
        let roll: f64 = rng.random();

        if player.sss_pity > 800 && player.inc_sss < 0.04 {
            player.inc_sss += ((player.sss_pity - 800) as f64 * 0.000099);
        } else if player.ss_pity > 170 {
            player.inc_ss += ((player.ss_pity - 170) as f64 * 0.005)
        } else if player.s_pity > 40 {
            player.s_pity += ((player.s_pity - 40) as f64 * 0.0083)
        }

        // 0.072% of hitting Mythic
        if roll < mythic {
            player.sss_pity = 0;
            return Rarities::MythicSSS;
        }
        // 0.48% chance of hitting SS
        else if roll < ss {
            player.ss_pity = 0;
            return Rarities::SS;
        }
        // 2.4% chance of hitting S
        else if roll < s {
            player.s_pity = 0;
            return Rarities::S;
        }
        // 7.2% chance of hitting A
        else if roll < a {
            return Rarities::A;
        }
        // 24% chance of hitting B
        else if roll < b {
            return Rarities::B;
        }
        // 73.608% chance of hitting C
        else {
            return Rarities::C;
        }
    }
}

pub fn get_string(item: Rarities) -> &'static str {
    match item {
        Rarities::MythicSSS => "Mythic",
        Rarities::SS => "SS",
        Rarities::S => "S",
        Rarities::A => "A",
        Rarities::B => "B",
        Rarities::C => "C",
        Rarities::NoTickets => "NoTickets",
    }
}
