use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)];
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = false)]
    Import {
        #[arg(required = true)]
        file_type: FileTypes,
        table: Tables,
        location: String,
    },
    Run,
}

#[derive(ValueEnum, SubCommand, Clone)]
pub enum FileTypes {
    Csv,
}

#[derive(ValueEnum, Subcommand, Clone)]
pub enum Tables {
    Allergies,
    Patients,
    Organizations,
    Providers,
}
