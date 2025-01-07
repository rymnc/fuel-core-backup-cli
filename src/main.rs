mod backup;
mod restore;

#[cfg(feature = "compress")]
mod compression;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum BackupCommand {
    Backup {
        #[clap(long)]
        backup_from: String,
        #[clap(long)]
        backup_to: String,
    },
    Restore {
        #[clap(long)]
        restore_to: String,
        #[clap(long)]
        restore_from: String,
    },
}

#[derive(Parser, Debug)]
#[clap(name = "fuel-db-backup")]
pub struct Args {
    #[clap(subcommand)]
    command: BackupCommand,
}

fn main() {
    let args = Args::parse();

    match args.command {
        BackupCommand::Backup {
            backup_from,
            backup_to,
        } => {
            backup::backup(&backup_from, &backup_to).unwrap();
        }
        BackupCommand::Restore {
            restore_from,
            restore_to,
        } => {
            restore::restore(&restore_to, &restore_from).unwrap();
        }
    }
}
