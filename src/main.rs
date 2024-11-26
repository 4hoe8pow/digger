use chrono::Utc;
use clap::{Parser, Subcommand};
use color_eyre::{eyre::Ok, Result};
use dapplication::interactors::terminal_interactor::TerminalInteractor;
use dinfrastructure::ticket_repository_impl::TicketRepositoryImpl;
use dpresentation::{
    controllers::terminal_controller::TerminalController,
    presenters::ratatui_presenter::RatatuiPresenter, table_colors::TableColors,
};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "Digger")]
#[command(version = "1.0")]
#[command(about = "Manage tickets using a TOML file", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { file_name: String },
    Run { file_name: String },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Cli::try_parse()?.command.execute()
}

trait Execute {
    fn execute(self) -> Result<()>;
}

impl Execute for Commands {
    fn execute(self) -> Result<()> {
        match self {
            Commands::New { file_name } => create_new_file(&file_name),
            Commands::Run { file_name } => run_terminal(&file_name),
        }
    }
}

/// Creates a new TOML file with predefined content
fn create_new_file(file_name: &str) -> Result<()> {
    let file_path = ensure_toml_extension(file_name);

    if Path::new(&file_path).exists() {
        println!("Warning: File '{}' already exists.", file_path);
    } else {
        let now = Utc::now().to_rfc3339();
        let content = format!(
            r#"
[[ticket_data]]
id = "000"
level = "One"
title = "No Issues"
status = "Pending"
created_at = "{}"
"#,
            now
        );
        fs::write(&file_path, content)?;
        println!("New file created: {}", file_path);
    }

    Ok(())
}

/// Runs the terminal controller
fn run_terminal(file_name: &str) -> Result<()> {
    let file_path = ensure_toml_extension(file_name);

    let repository = TicketRepositoryImpl::new(file_path.clone());
    let presenter = RatatuiPresenter::new(TableColors::new(), repository.count_tickets()?);

    TerminalController::new(TerminalInteractor::new(repository, presenter)?).run(ratatui::init())?;

    ratatui::restore();
    Ok(())
}

/// Ensures the file name has a .toml extension
fn ensure_toml_extension(file_name: &str) -> String {
    if Path::new(file_name).extension().is_some() {
        file_name.to_string()
    } else {
        format!("{}.toml", file_name)
    }
}
