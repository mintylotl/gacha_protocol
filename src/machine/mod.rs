mod lever;

pub use lever::{pull, Rarities};
pub struct PityCtx {
    a_pity: u16,
    s_pity: u16,
    sss_pity: u16,
    inc_a: f64,
    inc_s: f64,
    inc_sss: f64,
}

pub fn handle_pull(pity_ctx: &PityCtx) -> Rarities {
    lever::pull(pity_ctx)
}
