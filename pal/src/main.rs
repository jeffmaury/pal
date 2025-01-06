use clap::{Parser, Subcommand};
use pal-api

fn main() {
    let cli = Cli::parse();
    match (cli.command) {
        Commands::Models(models) => {
            processModelsCommand(models)
        }
        Commands::Recipes(recipes) => {
            processRecipesCommand(recipes)
        }
    }
}

fn processRecipesCommand(command: RecipesCommands) {
}

fn processModelsCommand(command: ModelsCommands) {
    match (command) {
        ModelsCommands::List => get_models()
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, name = "models")]
    Models(ModelsCommands),

    #[command(subcommand, name = "recipes")]
    Recipes(RecipesCommands),
}

#[derive(Parser)]
pub enum ModelsCommands {
    List,
    Pull
}

#[derive(Parser)]
pub enum RecipesCommands {
    List,
    Start,
    Stop
}
