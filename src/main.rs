extern crate brawllib_rs;
extern crate cgmath;
extern crate pf_sandbox;
extern crate ref_slice;
extern crate treeflection;
extern crate enum_traits;
extern crate getopts;

mod action_map;
mod cli;
mod export;

fn main() {
    if let Some(cli) = cli::parse_cli() {
        export::export(cli.mod_name, &cli.export_fighters);
    }
}
