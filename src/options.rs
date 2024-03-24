use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[clap(
    author = "Krypton (https://krypton.ninja)",
    about,
    arg_required_else_help(true)
)]
pub struct Options {
    /// Domain to scan for
    #[arg(short, long)]
    pub domain: String,

    #[clap(flatten, next_help_heading = "Passive DNS")]
    pub passive_dns: PassiveDNSOptions,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[group(skip)]
pub(crate) struct PassiveDNSOptions {
    #[clap(long)]
    /// Ignore expired certificates.
    pub passive_dns_ignore_expired: bool,

    #[clap(long)]
    /// Only care about the recently (24 hours) created certificates
    pub passive_dns_recent_only: bool,
}
