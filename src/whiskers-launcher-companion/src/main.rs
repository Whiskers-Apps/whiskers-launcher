// Prevents windows from opening a terminal window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Parser, Subcommand};
use features::{
    actions::cli_run_action, search::cli_search, settings::{cli_show_settings, cli_write_settings}
};

mod features;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    IndexApps {},
    IndexExtensions {},
    Search { search_text: Option<String> },
    RunAction{ action: String },
    WriteSettings { settings_json: String },
    GetSettings {},
    WriteResults { results_json: String },
    GetResults {},
    WriteFormRequest { request_json: String },
    GetFormRequest {},
    WriteExtensionFormResult { result_json: String },
    ReadExtensionFormResult {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::IndexApps {} => {}
        Commands::IndexExtensions {} => {}
        Commands::Search { search_text } => {
            if let Some(search_text) = search_text {
                cli_search(&search_text);
            } else {
                cli_search("");
            }
        }
        Commands::RunAction { action } => {
            cli_run_action(&action);
        }
        Commands::WriteSettings { settings_json } => {
            cli_write_settings(&settings_json);
        }
        Commands::GetSettings {} => {
            cli_show_settings();
        }
        Commands::WriteResults { results_json } => {}
        Commands::GetResults {} => {}
        Commands::WriteFormRequest { request_json } => {}
        Commands::GetFormRequest {} => {}
        Commands::WriteExtensionFormResult { result_json } => {}
        Commands::ReadExtensionFormResult {} => {}
    }
}
