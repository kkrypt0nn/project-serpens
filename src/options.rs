use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[clap(
    author = "Krypton (https://krypton.ninja)",
    version,
    about,
    arg_required_else_help(true)
)]
pub struct Options {
    /// Domain to scan for
    #[arg(short, long)]
    pub domain: String,
}
