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
    let mut s = 0.072;
    let mut a = 0.24;

    if pity_stats.sss_pity > 1104 {
        mythic += ((pity_stats.sss_pity - 1104) as f64 * 0.000099).min(0.04);
    }
    if pity_stats.s_pity > 24 {
        s += ((pity_stats.s_pity - 24) as f64 * 0.005).min(0.08);
    }
    if pity_stats.a_pity > 5 {
        a += ((pity_stats.a_pity - 5) as f64 * 0.1).min(0.5);
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
