mod machine;
use machine::{get_string, handle_pull};
pub use machine::{PityCtx, Rarities};

pub fn roll(ctx: &PityCtx) -> Rarities {
    handle_pull(ctx)
}
