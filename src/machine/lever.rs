use super::PityCtx;
use rand::{self, Rng};

pub enum Rarities {
    MythicSSS,
    S,
    A,
    B,
}
pub fn pull(pity_stats: &PityCtx) -> Rarities {
    let b_mythic = 0.00072;
    let b_s = b_mythic + 0.03;
    let b_a = b_s + 0.12;
    let b_b = 1.0 - (b_mythic + b_s + b_a);

    let mut rng = rand::rng();
    let roll: f64 = rng.random();
    let player = pity_stats;

    let mut inc_sss = 0.0;
    let mut inc_s = 0.0;
    let mut inc_a = 0.0;

    if player.sss_pity > 1104 {
        inc_sss += ((player.sss_pity - 1100) as f64 * 0.000099).min(0.04);
    }
    if player.a_pity > 12 {
        inc_a += ((player.a_pity - 12) as f64 * 0.005).min(0.08);
    }
    if player.s_pity > 24 {
        inc_s += ((player.s_pity - 24) as f64 * 0.0083).min(0.2);
    }

    let mythic = b_mythic + inc_sss;
    let s = b_s + inc_s;
    let a = b_a + inc_a;
    let b = b_b - (inc_sss + inc_s + inc_a);

    // 0.072% of hitting Mythic
    if roll < mythic {
        return Rarities::MythicSSS;
    } else if roll < s {
        return Rarities::S;
    } else if roll < a {
        return Rarities::A;
    } else {
        return Rarities::B;
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
