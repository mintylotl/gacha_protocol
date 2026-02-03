mod lever;

pub use lever::{pull, Rarities};
pub struct PityCtx {
    pub a_pity: u16,
    pub s_pity: u16,
    pub sss_pity: u16,
}

pub fn handle_pull(pity_ctx: &PityCtx) -> Rarities {
    pull(pity_ctx)
}

pub fn get_string(item: &Rarities) -> &'static str {
    lever::get_string(item)
}
