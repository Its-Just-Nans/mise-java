use clap::Subcommand;

mod arch;
mod os;
mod vendors;

#[derive(Debug, Subcommand)]
enum Commands {
    Arch(arch::Arch),
    Os(os::Os),
    Vendor(vendors::Vendor),
}

impl Commands {
    pub fn run(self) -> eyre::Result<()> {
        match self {
            Self::Arch(cmd) => cmd.run(),
            Self::Os(cmd) => cmd.run(),
            Self::Vendor(cmd) => cmd.run(),
        }
    }
}

/// Export JVM data
#[derive(Debug, clap::Args)]
pub struct Ls {
    #[clap(subcommand)]
    command: Commands,
}

impl Ls {
    pub fn run(self) -> eyre::Result<()> {
        self.command.run()
    }
}
