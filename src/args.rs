use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "beesag",
    version = "1.0",
    about = "Beesag: From Genomics to Machine, Deep Learning
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {}
