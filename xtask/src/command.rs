#[derive(Debug, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone, Copy)]
pub enum Command {
    #[command(about = "Runs the kernel")]
    Run,
}
