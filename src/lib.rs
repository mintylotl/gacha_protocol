mod machine;
use machine::handle_pull;
pub use machine::{PityCtx, Rarities};

pub fn roll(ctx: &PityCtx) -> Rarities {
    handle_pull(ctx)
}
