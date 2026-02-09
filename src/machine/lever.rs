use super::PityCtx;
use rand::Rng;

pub enum Rarities {
    MythicSSS,
    S,
    A,
    B,
}
pub fn pull(pity_stats: &PityCtx) -> Rarities {
    let mut rng = rand::rng();
    let mut roll: f64 = rng.random();

    let mut mythic = 0.00072;
    let mut s = 0.048;
    let mut a = 0.072;

    if pity_stats.sss_pity > 1104 {
        mythic += ((pity_stats.sss_pity - 1104) as f64 * 0.000024).min(0.01);
    }
    if pity_stats.s_pity > 24 {
        s += ((pity_stats.s_pity - 16) as f64 * 0.024).min(0.072);
    }
    if pity_stats.a_pity > 12 {
        a += ((pity_stats.a_pity - 12) as f64 * 0.048).min(0.24);
    }

    // 0.072% of hitting Mythic
    if roll < mythic {
        return Rarities::MythicSSS;
    }
    roll -= mythic;

    if roll < s {
        return Rarities::S;
    }
    roll -= s;

    if roll < a {
        return Rarities::A;
    }

    Rarities::B
}

pub fn _get_string(item: &Rarities) -> &'static str {
    match item {
        Rarities::MythicSSS => "Mythic",
        Rarities::S => "S",
        Rarities::A => "A",
        Rarities::B => "B",
    }
}
