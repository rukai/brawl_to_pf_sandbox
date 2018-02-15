extern crate pf_sandbox;
extern crate byteorder;

use std::fs;

use pf_sandbox::package::Package;
use pf_sandbox::fighter::Fighter;

pub mod parse;

fn main() {
    match fs::read_dir("data/brawl/fighter") {
        Ok(fighter_dir) => {
            let brawl_fighters = parse::fighters(fighter_dir);
            println!("fighters: {:#?}", brawl_fighters);

            let mut package = Package::open_or_generate("brawl").unwrap();
            package.fighters.clear();

            for brawl_fighter in brawl_fighters {
                let mut fighter = Fighter::default();
                fighter.name = brawl_fighter.cased_fighter_name.clone();
                package.fighters.push(brawl_fighter.cased_fighter_name, fighter);
            }

            package.meta.title = String::from("Brawl");
            package.save();
        }
        Err(_) => {
            println!("'data' directory incorrectly setup.");
        }
    }
}

