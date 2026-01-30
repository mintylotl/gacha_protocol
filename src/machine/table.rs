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

        let b_mythic = 0.00072;
        let b_ss = b_mythic + 0.0072;
        let b_s = b_ss + 0.024;
        let b_a = b_s + 0.072;
        let b_b = b_a + 0.24;
        let b_c = 1.0 - (b_mythic + b_ss + b_s + b_a + b_b);

        let mut rng = rand::rng();
        let roll: f64 = rng.random();

        if player.sss_pity > 1100 {
            player.inc_sss += ((player.sss_pity - 1100) as f64 * 0.000099).min(0.04);
        }
        if player.ss_pity > 188 {
            player.inc_ss += ((player.ss_pity - 188) as f64 * 0.005).min(0.08);
        } else if player.s_pity > 45 {
            player.inc_s += ((player.s_pity - 50) as f64 * 0.0083).min(0.2);
        }

        let mythic = 0.00072 + player.inc_sss;
        let ss = b_ss + player.inc_ss;
        let s = b_ss + player.inc_s;
        let a = s + 0.072;
        let b = a + 0.24;
        let c = b_c - (player.inc_sss + player.inc_ss + player.inc_s);

        // 0.072% of hitting Mythic
        if roll < mythic {
            player.sss_pity = 0;
            player.inc_sss = 0.0;
            return Rarities::MythicSSS;
        }
        // 0.48% chance of hitting SS
        else if roll < ss {
            player.ss_pity = 0;
            player.inc_ss = 0.0;
            return Rarities::SS;
        }
        // 2.4% chance of hitting S
        else if roll < s {
            player.s_pity = 0;
            player.inc_s = 0.0;
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

pub fn get_string(item: &Rarities) -> &'static str {
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
