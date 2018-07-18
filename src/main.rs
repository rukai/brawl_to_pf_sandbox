             extern crate brawllib_rs;
             extern crate cgmath;
             extern crate enum_traits;
             extern crate env_logger;
             extern crate getopts;
             extern crate noisy_float;
             extern crate pf_sandbox_lib;
             extern crate ref_slice;
             extern crate treeflection;
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
