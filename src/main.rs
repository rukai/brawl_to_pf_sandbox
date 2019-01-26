#[macro_use] extern crate log;

mod action_map;
mod cli;
mod export;
mod logger;

fn main() {
    logger::init();
    if let Some(cli) = cli::parse_cli() {
        export::export(cli.mod_name, &cli.export_fighters);
    }
}
