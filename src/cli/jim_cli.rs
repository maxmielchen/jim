use clap::{Parser, Subcommand}; 

/// A command line tool to help you create and manage your gitignore
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Jim 
{
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
pub enum Commands
{
    /// Adds a new template if it does not exists to the gitignore
    Add 
    {
        name: Vec<String>
    },

    /// List all templates used or optionally the excluded files with '-a'
    List
    {
        /// Show all excluded files and folders
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },

    /// Deletes a template if it exists to the gitignore
    Delete
    {
        name: Vec<String>
    },

    /// Searches for a template or sees if it exists
    Search
    {
        query: String
    },

    /// Checks every template that is available in the gitignore and updates it if necessary
    Update,
}