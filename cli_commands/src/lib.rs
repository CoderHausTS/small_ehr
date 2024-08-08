use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct JsonCli {
    #[command(subcommand)]
    pub import_commands: ImportCommands,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct RestCli {
    #[command(subcommand)]
    pub rest_commands: RestApiCommands,
}

#[derive(Subcommand)]
pub enum ImportCommands {
    #[command(arg_required_else_help = false)]
    Import {
        #[arg(required = true)]
        file_type: FileTypes,
        table: Tables,
        location: String,
    },
    Run,
}

#[derive(Subcommand)]
pub enum RestApiCommands {
    #[command(arg_required_else_help = false)]
    Run,
}


#[derive(ValueEnum, Subcommand, Clone)]
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
