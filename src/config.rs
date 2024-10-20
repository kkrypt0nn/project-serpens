use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, Parser)]
#[clap(
    author = "Krypton (https://krypton.ninja)",
    about,
    arg_required_else_help(true)
)]
pub struct Config {
    /// Domain to scan for
    #[arg(short, long)]
    pub domain: String,

    #[clap(flatten, next_help_heading = "Enumerate Files")]
    pub enumerate_files: EnumerateFilesConfig,

    #[clap(flatten, next_help_heading = "Enumerate Subdomains")]
    pub enumerate_subdomains: EnumerateSubdomainsConfig,

    #[clap(flatten, next_help_heading = "Passive DNS")]
    pub passive_dns: PassiveDNSConfig,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[group(skip)]
pub(crate) struct PassiveDNSConfig {
    #[clap(long, default_value_t = false)]
    /// Ignore expired certificates.
    pub passive_dns_ignore_expired: bool,

    #[clap(long, default_value_t = false)]
    /// Only care about the recently (24 hours) created certificates
    pub passive_dns_recent_only: bool,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[group(skip)]
pub(crate) struct EnumerateSubdomainsConfig {
    #[clap(long, default_value = "")]
    /// The path to the wordlist to use
    pub enumerate_subdomains_wordlist: String,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, Default)]
#[group(skip)]
pub(crate) struct EnumerateFilesConfig {
    #[clap(long, default_value = "")]
    /// The path to the wordlist to use
    pub enumerate_files_wordlist: String,
    #[clap(long, default_value = "")]
    /// The extension to append to the file names
    pub enumerate_files_extension: String,
}
