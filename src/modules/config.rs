use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-system-monitor")]
#[command(about = "Simple system monitor")]
#[command(version)]
pub struct Config {
    /// update interval in millis
    #[arg(short, long, default_value = "200")]
    pub interval: u64,

    /// dont show network usage
    #[arg(long)]
    pub no_network: bool,

    /// dont show disk usage
    #[arg(long)]
    pub no_disk: bool,
}
