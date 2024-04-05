use clap::{Parser, ValueHint};

#[derive(Parser)]
pub struct Cli {
    /// The notification text to be sent
    #[arg(
        long,
        env = "TEXT",
        value_hint = ValueHint::Other,
        default_value = "Hello world",
    )]
    pub text: String,

    #[command(flatten)]
    pub notifications: pling::clap::Args,

    /// Unused in code. Its just there to check that the clap headings work correctly
    #[arg(long)]
    pub after: bool,
}

fn main() -> anyhow::Result<()> {
    let matches = Cli::parse();
    matches.notifications.send_ureq(&matches.text)?;
    Ok(())
}
