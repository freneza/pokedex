mod api;
mod cli;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
extern crate clap;
extern crate serde;

use clap::{Command};
use repositories::pokemon::InMemoryRepository;
use std::sync::Arc;

fn cli() -> Command {
    Command::new("pokedex")
        .about("Pokemon Deck Manager")
        .subcommand_required(false)
        .subcommand(
            Command::new("cli")
            .about("Runs in CLI mode")
        )
}

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("cli", _)) => cli::run(repo),
        _ => api::serve("localhost:8000", repo),
    }
}
